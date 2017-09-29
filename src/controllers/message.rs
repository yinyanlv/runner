use iron::prelude::*;

use common::http::*;

pub fn render_unread_message(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    respond_view("message", &data)
}
