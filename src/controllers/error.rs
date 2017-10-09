use iron::prelude::*;

use common::http::*;

pub fn render_not_found(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("title", json!("此页面不存在"));

    respond_view("error", &data)
}

pub fn render_forbidden(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("title", json!("禁止访问"));

    respond_view("error", &data)
}