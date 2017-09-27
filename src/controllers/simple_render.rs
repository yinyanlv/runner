use iron::prelude::*;

use common::http::*;

pub fn render_resource(req: &mut Request) -> IronResult<Response> {

    respond_view("resource", &ViewData::new(req))
}

pub fn render_about_site(req: &mut Request) -> IronResult<Response> {

    respond_view("about-site", &ViewData::new(req))
}