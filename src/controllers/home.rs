use iron::prelude::*;
use serde_json::value::Value;

use core::http::*;

pub fn render_home(req: &mut Request) -> IronResult<Response> {

    let mut data = ResponseData::new(req);

    data.insert("title", Value::String("runner".to_owned()));
    data.insert("message", Value::String("欢迎你，这里是首页".to_owned()));

    respond_view("home/index", &data)
}


