use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;
use hyper::Client;

use url::Url;
use persistent::Read;

use core::config::Config;
use core::http::*;
use core::utils::*;
use services::user::*;

pub fn render_login(req: &mut Request) -> IronResult<Response> {

    respond_view("login/index", &ResponseData::new(req))
}

pub fn login(req: &mut Request) -> IronResult<Response> {

    let session = req.session().get::<SessionObject>().unwrap();
    let params = get_request_body(req);
    let pool = get_mysql_pool(req);
    let username = &params.get("username").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let user_id_wrapper = check_user_login(&pool, username, password);

    if user_id_wrapper.is_none() {  // 登录失败，该用户不存在！

        return redirect_to("http://localhost:3000/register");
    }

    let user_id = user_id_wrapper.unwrap();

    req.session().set(SessionObject {
        username: username.to_string()
    });

    redirect_to("http://localhost:3000")
}

pub fn github_auth_callback(req: &mut Request) -> IronResult<Response> {

    let params = get_request_query(req);
    let code = &params.get("code").unwrap()[0];
    let config = req.get::<Read<Config>>().unwrap().value();
    let github_config = config.get("github").unwrap().as_table().unwrap();
    let client_id = github_config.get("client_id").unwrap().as_str().unwrap();
    let client_secret = github_config.get("client_secret").unwrap().as_str().unwrap();
    let mut url = Url::parse("https://github.com/login/oauth/access_token").unwrap();

    url.query_pairs_mut()
        .append_pair("code", &code)
        .append_pair("client_id", &client_id)
        .append_pair("client_secret", &client_secret);

    let client = Client::new();
    let mut token = String::new();
    client.post(url).send().unwrap().read_to_string(&mut token).unwrap();

    println!("{:?}", token);

    redirect_to("http://localhost:3000")
}

