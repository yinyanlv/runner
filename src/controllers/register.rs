use std::collections::HashMap;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::Url;
use hbs::Template;
use hbs::handlebars::to_json;
use serde_json::value::{Value, Map};
use urlencoded::{UrlEncodedBody};
use persistent::Read;

use core::db::MySqlPool;
use core::utils::*;

pub fn render_register(_req: &mut Request) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(Template::new("register/index", ""));

    Ok(res)
}

pub fn register(req: &mut Request) -> IronResult<Response> {

    let params = req.get::<UrlEncodedBody>().unwrap();
    let username = &params.get("username").unwrap()[0];
    let email = &params.get("email").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let salt = gen_salt();
    let password_with_salt = password.to_string() + &*salt;
    let password_hash = gen_md5(&password_with_salt);
    let create_time = gen_datetime();
    let pool = req.get::<Read<MySqlPool>>().unwrap().value();
    let mut stmt = pool.prepare("INSERT INTO user (username, email, password, salt, create_time) VALUES (?, ?, ?, ?, ?)").unwrap();
    let result = stmt.execute((username, email, password_hash, salt, create_time)).unwrap();
    let url = Url::parse("http://localhost:3000").unwrap();

    let res = Response::with((status::Found, Redirect(url)));

    Ok(res)
}