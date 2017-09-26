use iron::prelude::*;

use common::http::*;
use common::utils::*;
use services::user::*;

pub fn render_register(req: &mut Request) -> IronResult<Response> {

    respond_view("user/register", &ViewData::new(req))
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