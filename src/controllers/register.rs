use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::Url;
use hbs::Template;
use hbs::handlebars::to_json;
use serde_json::value::{Value, Map};
use urlencoded::{UrlEncodedBody, UrlEncodedQuery};
use persistent::Read;
use chrono::*;
use crypto::md5::*;
use rand::*;

use core::db::MySqlPool;
use core::utils::mapping;

pub fn render_register(_req: &mut Request) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(Template::new("register/index", ""));

    Ok(res)
}

pub fn register(req: &mut Request) -> IronResult<Response> {

    let mut params = req.get::<UrlEncodedBody>().unwrap();
    let username = params.get("username").unwrap()[0].clone();
    let email = params.get("email").unwrap()[0].clone();
    let password = params.get("password").unwrap()[0].clone();
    let salt = thread_rng()
                .gen_ascii_chars()
                .take(32)
                .collect::<String>();
    let password_with_salt = password.to_string() + &*salt;
    let mut sh = Md5::new();
    sh.input_str(&password_with_salt);
    let salt_hash = sh.result_str();
    let create_time = Local::now().naive_local();
    let pool = req.get::<Read<MySqlPool>>().unwrap().value();
    let mut stmt = pool.prepare("INSERT INTO user (username, email, password, salt, create_time) VALUES (?, ?, ?, ?, ?)").unwrap();
    let result = stmt.execute((username, email, password, salt, create_time)).unwrap();
    let url = Url::parse("http://localhost:3000").unwrap();

    let res = Response::with((status::Found, Redirect(url)));

    Ok(res)
}