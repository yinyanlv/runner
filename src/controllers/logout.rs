use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;

use common::http::*;

pub fn logout(req: &mut Request) -> IronResult<Response> {

    req.session().clear().unwrap();

    redirect_to("/")
}