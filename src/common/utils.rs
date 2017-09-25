use std::collections::HashMap;

use iron::prelude::*;
use rand::*;
use crypto::md5::Md5;
use crypto::digest::Digest;  // used for input_str, result_str
use chrono::{Local, NaiveDateTime};
use persistent::Read;
use mysql::Pool;
use urlencoded::{UrlEncodedQuery, UrlEncodedBody};
use toml::value::Table;
use serde::Serialize;
use serde_json::{self, Value};
use hbs::handlebars::{Helper, Handlebars, RenderContext, RenderError};
use hbs::handlebars::to_json;

use common::config::Config;
use common::db::MySqlPool;

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

pub fn gen_datetime() -> NaiveDateTime {

    Local::now().naive_local()
}

pub fn get_config(req: &mut Request) -> Table {

    req.get::<Read<Config>>().unwrap().value()
}

pub fn get_mysql_pool(req: &mut Request) -> Pool {

    req.get::<Read<MySqlPool>>().unwrap().value()
}

pub fn get_request_body(req: &mut Request) -> HashMap<String, Vec<String>> {

    req.get::<UrlEncodedBody>().unwrap()
}

pub fn get_request_query(req: &mut Request) -> HashMap<String, Vec<String>> {

    req.get::<UrlEncodedQuery>().unwrap()
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