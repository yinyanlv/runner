use iron::prelude::*;
use iron::status::*;

pub fn render_index(req: &mut Request) -> IronResult<Response> {

    Ok(Response::with((OK, "hello rust! let's begin...")))
}