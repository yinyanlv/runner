use iron::prelude::*;
use iron::status;
use iron::Url;
use iron::modifiers::Redirect;
use hbs::Template;
use hbs::handlebars::to_json;
use serde_json::value::{Value, Map};
use urlencoded::UrlEncodedBody;
use persistent::Read;
use mysql::*;

use core::db::MySqlPool;
use core::utils::*;

pub fn render_login(_req: &mut Request) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(Template::new("login/index", ""));

    Ok(res)
}

pub fn login(req: &mut Request) -> IronResult<Response> {

    let params = req.get::<UrlEncodedBody>().unwrap();
    let pool = req.get::<Read<MySqlPool>>().unwrap().value();
    let username = &params.get("username").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let user_id_wrapper = check_login(&pool, username, password);

    if user_id_wrapper.is_none() {

        println!("登录失败，该用户不存在！");

        let url = Url::parse("http://localhost:3000/register").unwrap();
        let res = Response::with((status::Found, Redirect(url)));

        return Ok(res);
    }

    let user_id = user_id_wrapper.unwrap();

    let url = Url::parse("http://localhost:3000").unwrap();
    let res = Response::with((status::Found, Redirect(url)));

    Ok(res)
}

pub fn check_login(pool: &Pool, username: &str, password: &str) -> Option<u32> {

    let mut result = pool.prep_exec("SELECT id, password, salt FROM user where username = ?", (username,)).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (user_id, password_hash, salt) = from_row::<(u32, String, String)>(row);
    let password_with_salt = password.to_string() + &*salt;

    if password_hash != gen_md5(&password_with_salt) {
        return None;
    }

    Some(user_id)
}

