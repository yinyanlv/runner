use iron::prelude::*;

use common::http::*;

pub fn render_not_found(req: &mut Request) -> IronResult<Response> {

    respond_view("404", &ViewData::new(req))
}