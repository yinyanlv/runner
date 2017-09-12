use iron::prelude::*;

use core::http::*;
use core::utils::*;
use services::user::*;

pub fn render_register(req: &mut Request) -> IronResult<Response> {

    respond_view("register/index", &ViewData::new(req))
}

pub fn register(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let username = &params.get("username").unwrap()[0];
    let email = &params.get("email").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let salt = gen_salt();
    let password_with_salt = password.to_string() + &*salt;
    let password_hashed = gen_md5(&password_with_salt);
    let create_time = gen_datetime();
    let pool = get_mysql_pool(req);
    let values = (username.to_owned(), email.to_owned(), password_hashed, salt, create_time);
    let result = create_user(&pool, values);

    if result.is_none() {

        let mut data = JsonData::new();

        data.message = "该用户名已被注册！".to_owned();

        return respond_json(&data);
    }

    redirect_to("http://localhost:3000")
}