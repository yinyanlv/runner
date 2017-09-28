use std::io::Read;

use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;
use hyper::header::UserAgent;
use serde_json::{Value};
use url::{Url, form_urlencoded};

use common::http::*;
use common::utils::*;
use common::lazy_static::{HTTPS_CLIENT, CONFIG_TABLE};
use services::user::*;
use controllers::home::render_home;

pub fn render_login(req: &mut Request) -> IronResult<Response> {

    respond_view("login", &ViewData::new(req))
}

pub fn login(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let username = &params.get("username").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let username_wrapper = check_user_login(username, password);

    let mut data = JsonData::new();

    if username_wrapper.is_none() {

        data.success = false;
        data.message = "登录失败，用户名或密码不正确！".to_owned();

        return respond_json(&data);
    }

    let username = username_wrapper.unwrap();
    let user = get_user(&*username).unwrap();

    req.session().set(SessionData {
        user: json_stringify(&user)
    });

    data.data = json!("/");

    respond_json(&data)
}

pub fn github_auth_callback(req: &mut Request) -> IronResult<Response> {

    let params = get_request_query(req);
    let code = &params.get("code").unwrap()[0];
    let github_config = CONFIG_TABLE.get("github").unwrap().as_table().unwrap();
    let client_id = github_config.get("client_id").unwrap().as_str().unwrap();
    let client_secret = github_config.get("client_secret").unwrap().as_str().unwrap();

    let access_token = get_github_access_token(&code, &client_id, &client_secret);

    let mut user_info = get_github_user_info(&access_token);
    let id = user_info["id"].as_u64().unwrap();

    let username_wrapper;

    if is_github_user_binded(id) {  // 该用户已绑定

        username_wrapper = update_github_user(&user_info);
    } else {

        username_wrapper = bind_github_user(&user_info);
    }

    if username_wrapper.is_some() {

        let username = username_wrapper.unwrap();

        let user = get_user(&*username).unwrap();

        req.session().set(SessionData {
            user: json_stringify(&user)
        });

        redirect_to("/")
    } else {  // 该github用户名已被本站用户注册

        let username = user_info["login"].as_str().unwrap();

        let mut data = ViewData::new(req);
        data.insert("username", json!(username));
        data.insert("message", json!("该github用户名已被本站用户注册，请填写新的用户名后，点击绑定"));
        data.insert("user_info", json!(json_stringify(&user_info)));

        respond_view("bind-user", &data)
    }
}

fn get_github_access_token(code: &str, client_id: &str, client_secret: &str) -> String {

    let mut url = Url::parse("https://github.com/login/oauth/access_token").unwrap();
    url.query_pairs_mut()
        .append_pair("code", code)
        .append_pair("client_id", client_id)
        .append_pair("client_secret", client_secret);

    let mut body = String::new();
    HTTPS_CLIENT.get(url.as_str()).send().unwrap().read_to_string(&mut body).unwrap();

    let mut access_token = String::new();
    for (key, value) in form_urlencoded::parse(body.as_bytes()).into_owned() {
        if key == "access_token" {
            access_token = value;
        }
    }

    access_token
}

fn get_github_user_info( access_token: &str) -> Value {

    let mut url = Url::parse("https://api.github.com/user").unwrap();
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let mut body = String::new();
    HTTPS_CLIENT.get(url.as_str())
        .header(UserAgent("runner".to_string()))  // UserAgent必须指定，但值可以为任意值
        .send()
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();

    json_parse(&*body)
}