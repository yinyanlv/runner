use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;

use common::http::*;
use common::utils::*;
use common::lazy_static::GITHUB_LOGIN_PATH;

use services::user::*;

pub fn render_register(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("github_login_path", json!(GITHUB_LOGIN_PATH.to_string()));

    respond_view("register", &data)
}

pub fn register(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let username = &params.get("username").unwrap()[0];
    let email = &params.get("email").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let avatar_url = gen_gravatar_url(email);
    let salt = gen_salt();
    let password_with_salt = password.to_string() + &*salt;
    let password_hashed = gen_md5(&password_with_salt);
    let create_time = gen_datetime().to_string();
    let obj = json!({
        "username": username.to_owned(),
        "email": email.to_owned(),
        "avatar_url": avatar_url,
        "github_account": "".to_owned(),
        "password_hashed": password_hashed,
        "salt": salt,
        "register_source": 0,
        "create_time": create_time
    });

    let result = create_user(&obj);

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "该用户名或邮箱已被注册！".to_owned();

        return respond_json(&data);
    }

    data.data = json!("/login");

    respond_json(&data)
}

/// 绑定github用户
pub fn bind_user(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let username_str = &params.get("username").unwrap()[0];
    let user_info_str = &params.get("userInfo").unwrap()[0];

    let mut data = JsonData::new();

    if is_user_created(username_str) {

        data.success = false;
        data.message = "该用户名已被注册！".to_owned();

        return respond_json(&data);
    }

    let mut user_info_obj = json_parse(user_info_str);

    user_info_obj["login"] = json!(username_str);

    let username_wrapper = bind_github_user(&user_info_obj);

    let username = username_wrapper.unwrap();

    let user = get_user(&*username).unwrap();

    req.session().set(SessionData {
        user: json_stringify(&user)
    }).unwrap();

    data.data = json!("/");

    respond_json(&data)
}