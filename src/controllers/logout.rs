use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;

use core::http::*;

pub fn logout(req: &mut Request) -> IronResult<Response> {

    req.session().clear();
    redirect_to("http://localhost:3000/login")
}