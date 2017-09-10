use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;

use core::http::*;
use core::utils::*;
use services::user::*;

pub fn render_login(req: &mut Request) -> IronResult<Response> {

    respond_view("login/index", &ResponseData::new(req))
}

pub fn login(req: &mut Request) -> IronResult<Response> {

    let session = req.session().get::<SessionObject>().unwrap();
    let params = get_request_body(req);
    let pool = get_mysql_pool(req);
    let username = &params.get("username").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let user_id_wrapper = check_user_login(&pool, username, password);

    if user_id_wrapper.is_none() {  // 登录失败，该用户不存在！

        return redirect_to("http://localhost:3000/register");
    }

    let user_id = user_id_wrapper.unwrap();

    req.session().set(SessionObject {
        username: username.to_string()
    });

    redirect_to("http://localhost:3000")
}

pub fn github_auth_callback(req: &mut Request) -> IronResult<Response> {

    let params = get_request_query(req);

    println!("{:?}", params);

    redirect_to("http://localhost:3000")
}

