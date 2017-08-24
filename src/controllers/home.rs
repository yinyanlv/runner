use iron::prelude::*;
use iron::status;

pub fn render_home(_req: &mut Request) -> IronResult<Response> {

    Ok(Response::with((status::Ok, "hello rust! let's begin...")))
}