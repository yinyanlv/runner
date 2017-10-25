use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;

use iron::prelude::*;
use iron_sessionstorage::Value as SessionValue;
use iron_sessionstorage::traits::SessionRequestExt;
use rand::*;
use crypto::md5::Md5;
use crypto::digest::Digest;  // used for input_str, result_str
use chrono::{Local, NaiveDateTime};
use urlencoded::{UrlEncodedQuery, UrlEncodedBody};
use serde::Serialize;
use serde_json::{self, Value};
use hbs::handlebars::{Helper, Handlebars, RenderContext, RenderError};
use router::{Router, Params};
use pulldown_cmark::{Parser, html};
use toml::value::Value::String as Toml_String;

use common::http::SessionData;
use common::lazy_static::{RECORDS_COUNT_PER_PAGE, ADMINS};

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

pub fn get_file_ext(filename: &str) -> Option<&str>{

    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn gen_gravatar_url(email: &str) -> String {

    "http://www.gravatar.com/avatar/".to_string() + &*gen_md5(email) + "?s=150"
}

pub fn gen_datetime() -> NaiveDateTime {

    Local::now().naive_utc()
}

pub fn check_and_get_string(value: &Value) -> String {

    if value.is_null() {

        "".to_string()
    } else {

        value.as_str().unwrap().to_string()
    }
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

    if req.get::<UrlEncodedQuery>().is_err() {

        false
    } else {

        true
    }
}

pub fn get_query_page(req: &mut Request) -> u32 {

    let has_query_params = has_request_query(req);
    let page: u32;

    if has_query_params {

        let query = get_request_query(req);

        if query.get("page").is_none()  {
            page = 1;
        } else {

            let page_wrapper = query.get("page").unwrap()[0].parse::<u32>();

            if page_wrapper.is_err() {

                page = 1;
            } else {

                page = page_wrapper.unwrap();
            }
        }
    } else {
        page = 1;
    }

    page
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

pub fn build_pagination(cur_page: u32, total: u32, base_url: &str) -> Value {

    let mut delta = 1;

    if total % RECORDS_COUNT_PER_PAGE == 0 {
        delta = 0;
    }

    let page_count = total / RECORDS_COUNT_PER_PAGE + delta;
    let mut is_show_prev_ellipsis = true;
    let mut is_show_next_ellipsis = true;
    let mut is_first_page_disabled = false;
    let mut is_last_page_disabled = false;
    let mut page_list;

    if page_count < 6 {

        is_show_prev_ellipsis = false;
        is_show_next_ellipsis = false;
    } else {

        if cur_page < 4  {

            is_show_prev_ellipsis = false;
        }

        if cur_page > page_count - 3 {

            is_show_next_ellipsis = false;
        }
    }

    if cur_page == 1 {

        is_first_page_disabled = true;
    }

    if cur_page == page_count {

        is_last_page_disabled = true;
    }

    page_list = vec![];

    if page_count < 6 {  // 总页数小于等于5时

        for i in 1..(page_count + 1) {

            if i == cur_page {

                page_list.push(json!({
                    "page": i,
                    "is_active": true
                }));
            } else {

                page_list.push(json!({
                    "page": i
                }));
            }
        }
    } else if cur_page < 4 {  // 总页数大于5，当前页码小于等于3时，隐藏左侧ellipsis

        for i in 1..6 {

            if i == cur_page {

                page_list.push(json!({
                    "page": i,
                    "is_active": true
                }));
            } else {

                page_list.push(json!({
                    "page": i
                }));
            }
        }
    } else if cur_page > page_count - 3 {  // 总页数大于5，当前页码距离总页数小于等于3时，隐藏右侧ellipsis

        for i in (page_count - 4)..(page_count + 1) {

            if i == cur_page {

                page_list.push(json!({
                    "page": i,
                    "is_active": true
                }));
            } else {

                page_list.push(json!({
                    "page": i
                }));
            }
        }

    } else {  // 当前页码的左右两侧各放置两个页码

        for i in (cur_page - 2)..(cur_page + 3) {

            if i == cur_page {

                page_list.push(json!({
                    "page": i,
                    "is_active": true
                }));
            } else {

                page_list.push(json!({
                    "page": i
                }));
            }
        }
    }

    json!({
        "base_url": base_url.to_owned(),
        "is_show_prev_ellipsis": is_show_prev_ellipsis,
        "is_show_next_ellipsis": is_show_next_ellipsis,
        "page_list": page_list,
        "is_first_page_disabled": is_first_page_disabled,
        "is_last_page_disabled": is_last_page_disabled,
        "first_page": 1,
        "last_page": page_count
    })
}

pub fn is_admin(username: &str) -> bool {

    if ADMINS.contains(&Toml_String(username.to_string())) {
        true
    } else {
        false
    }
}

#[test]
fn test_gen_salt() {

    assert_ne!(gen_salt(), "runner".to_owned());
}

#[test]
fn test_gen_md5() {

    assert_ne!(gen_md5("runner"), "runner".to_owned());
}

#[test]
fn test_get_file_ext() {

    assert_eq!(get_file_ext("abc.txt").unwrap(), "txt");
}