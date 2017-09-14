use iron::prelude::*;

use common::http::*;

pub fn render_home(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("title", json!("runner"));
    data.insert("message", json!("欢迎你，这里是首页"));

    println!("home");

    respond_view("home/index", &data)
}


