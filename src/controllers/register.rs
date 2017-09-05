use iron::prelude::*;
use iron::status;
use hbs::Template;
use hbs::handlebars::to_json;
use serde_json::value::{Value, Map};
use urlencoded::{UrlEncodedBody, UrlEncodedQuery};
use persistent::Read;
use chrono::*;

use core::db::MySqlPool;

pub fn render_register(_req: &mut Request) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(Template::new("register/index", ""))
        .set_mut(status::Ok);

    Ok(res)
}

pub fn register(req: &mut Request) -> IronResult<Response> {

    let mut res = Response::new();
    let mut params = req.get::<UrlEncodedBody>().unwrap();
    let username = params.get("username").unwrap();
    let email = params.get("email").unwrap();
    let password = params.get("password").unwrap();
    let salt = "test";
    let time = Local::now().naive_local();
    let pool = req.get::<Read<MySqlPool>>().unwrap().value();
    let stmt = pool.prepare("INSERT INTO user (username, email, password, salt, create_time) VALUES (?, ?, ?, ?, ?)").unwrap();
    let result = stmt.execute((username, email, password, salt, time));

    res.set_mut("success")
        .set_mut(status::Ok);

    Ok(res)
}