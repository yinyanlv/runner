use std::collections::HashMap;

use iron::prelude::*;
use rand::*;
use crypto::md5::Md5;
use crypto::digest::Digest;  // used for input_str, result_str
use chrono::Local;
use persistent::Read;
use mysql::Pool;
use urlencoded::UrlEncodedBody;
use toml::value::Table;

use core::config::Config;
use core::db::MySqlPool;

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

pub fn gen_datetime() -> String {

    Local::now().naive_local().to_string()
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