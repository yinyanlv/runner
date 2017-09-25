use std::io::Read;

use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;
use hyper::Client;
use hyper::header::UserAgent;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::{Value};
use url::{Url, form_urlencoded};

use common::config::Config;
use common::http::*;
use common::utils::*;
use services::user::*;

pub fn render_login(req: &mut Request) -> IronResult<Response> {

    respond_view("user/login", &ViewData::new(req))
}

pub fn login(req: &mut Request) -> IronResult<Response> {

    let session = req.session().get::<SessionData>().unwrap();
    let params = get_request_body(req);
    let pool = get_mysql_pool(req);
    let username = &params.get("username").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let user_wrapper = check_user_login(&pool, username, password);

    let mut data = JsonData::new();

    if user_wrapper.is_none() {

        data.success = false;
        data.message = "登录失败，用户名或密码不正确！".to_owned();

        return respond_json(&data);
    }

    let user = user_wrapper.unwrap();

    req.session().set(SessionData {
        user: json_stringify(&user)
    });

    data.data = json!("/");

    respond_json(&data)
}

lazy_static! {
    static ref HTTPS_CLIENT: Client = {

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        Client::with_connector(connector)
    };
}

pub fn github_auth_callback(req: &mut Request) -> IronResult<Response> {

    let params = get_request_query(req);
    let code = &params.get("code").unwrap()[0];
    let config = get_config(req);
    let github_config = config.get("github").unwrap().as_table().unwrap();
    let client_id = github_config.get("client_id").unwrap().as_str().unwrap();
    let client_secret = github_config.get("client_secret").unwrap().as_str().unwrap();
    let pool = get_mysql_pool(req);

    let client = &HTTPS_CLIENT;

    let access_token = get_github_access_token(&client, &code, &client_id, &client_secret);

    let user_info = get_github_user_info(&client, &access_token);
    let id = user_info["id"].as_u64().unwrap();
    let bind_time = gen_datetime().to_string();

    let user_wrapper = bind_github_user(&pool, &user_info);

    let user = user_wrapper.unwrap();

    req.session().set(SessionData {
        user: json_stringify(&user)
    });

    redirect_to("/")
}

fn get_github_access_token(client: &Client, code: &str, client_id: &str, client_secret: &str) -> String {

    let mut url = Url::parse("https://github.com/login/oauth/access_token").unwrap();
    url.query_pairs_mut()
        .append_pair("code", code)
        .append_pair("client_id", client_id)
        .append_pair("client_secret", client_secret);

    let mut body = String::new();
    client.get(url.as_str()).send().unwrap().read_to_string(&mut body).unwrap();

    let mut access_token = String::new();
    for (key, value) in form_urlencoded::parse(body.as_bytes()).into_owned() {
        if key == "access_token" {
            access_token = value;
        }
    }

    access_token
}

fn get_github_user_info(client: &Client, access_token: &str) -> Value {

    let mut url = Url::parse("https://api.github.com/user").unwrap();
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let mut body = String::new();
    client.get(url.as_str())
        .header(UserAgent("runner".to_string()))  // UserAgent必须指定，但值可以为任意值
        .send()
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();

    json_parse(&*body)
}