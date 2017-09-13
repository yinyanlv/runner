use iron::prelude::*;

use common::http::*;

pub fn render_not_found(req: &mut Request) -> IronResult<Response> {

    respond_view("error/404", &ViewData::new(req))
}