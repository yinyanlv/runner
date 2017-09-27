use iron::prelude::*;

use common::http::*;

pub fn render_create_topic(req: &mut Request) -> IronResult<Response> {
    let mut data = ViewData::new(req);

    data.insert("title", json!("发布话题"));
    respond_view("topic-editor", &data)
}

pub fn render_edit_topic(req: &mut Request) -> IronResult<Response> {
    let mut data = ViewData::new(req);

    data.insert("title", json!("编辑话题"));
    respond_view("topic-editor", &data)
}
