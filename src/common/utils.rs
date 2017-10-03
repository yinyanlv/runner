use std::collections::HashMap;

use iron::prelude::*;
use iron_sessionstorage::Value as SessionValue;
use iron_sessionstorage::traits::SessionRequestExt;
use rand::*;
use crypto::md5::Md5;
use crypto::digest::Digest;  // used for input_str, result_str
use chrono::{Local, NaiveDateTime};
use mysql::Pool;
use urlencoded::{UrlEncodedQuery, UrlEncodedBody};
use serde::Serialize;
use serde_json::{self, Value};
use hbs::handlebars::{Helper, Handlebars, RenderContext, RenderError, to_json};
use router::{Router, Params};
use pulldown_cmark::{Parser, html};

use common::http::SessionData;

pub fn parse_to_html(text: &str) -> String {

    let mut temp = String::new();
    let parser = Parser::new(text);

    html::push_html(&mut temp, parser);

    temp
}

pub fn gen_salt() -> String {

    thread_rng()
        .gen_ascii_chars()
        .take(32)
        .collect::<String>()
}

pub fn gen_md5(str: &str) -> String {

    let mut sh = Md5::new();

    sh.input_str(str);
    sh.result_str().to_string()
}

pub fn gen_gravatar_url(email: &str) -> String {

    "http://www.gravatar.com/avatar/".to_string() + &*gen_md5(email) + "?s=150"
}

pub fn gen_datetime() -> NaiveDateTime {

    Local::now().naive_local()
}

pub fn is_login(req: &mut Request) -> bool {

    let session_wrapper = req.session().get::<SessionData>().unwrap();

    if session_wrapper.is_some() {
        true
    } else {
        false
    }
}

pub fn get_session_obj(req: &mut Request) -> Value {

    let session_wrapper = req.session().get::<SessionData>().unwrap();

    json_parse(&*session_wrapper.unwrap().into_raw())
}

pub fn get_router_params(req: &mut Request) -> Params {

    req.extensions.get::<Router>().unwrap().clone()
}

pub fn get_request_body(req: &mut Request) -> HashMap<String, Vec<String>> {

    req.get::<UrlEncodedBody>().unwrap()
}

pub fn get_request_query(req: &mut Request) -> HashMap<String, Vec<String>> {

    req.get::<UrlEncodedQuery>().unwrap()
}

pub fn has_request_query(req: &mut Request) -> bool {

    true
//    if req.get::<UrlEncodedQuery>().is_none() {
//
//        false
//    } else {
//
//        true
//    }
}

pub fn json_stringify<T: Serialize>(data: &T) -> String {

    serde_json::to_string(data).unwrap()
}

pub fn json_parse(data: &str) -> Value {

    serde_json::from_str(data).unwrap()
}

pub fn mount_template_var(helper: &Helper, _: &Handlebars, context: &mut RenderContext) -> Result<(), RenderError> {

    let param_key = helper.param(0);
    let param_value = helper.param(1);

    if param_key.is_none() || param_value.is_none() {

        return Ok(());
    }

    let key = param_key.unwrap().value().as_str().unwrap().to_string();
    let value = param_value.unwrap().value();

    let mut view_data = context.context_mut().data_mut().as_object_mut().unwrap();

    view_data.insert(key, json!(value));

    Ok(())
}