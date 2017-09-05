use iron::prelude::*;
use iron::status;
use hbs::Template;
use hbs::handlebars::to_json;
use serde_json::value::{Value, Map};

pub fn render_login(_req: &mut Request) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(Template::new("login/index", ""));

    Ok(res)
}