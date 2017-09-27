use iron::prelude::*;
use iron_sessionstorage::traits::SessionRequestExt;

use common::http::*;
use common::utils::*;
use services::user::*;

pub fn render_user(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let username = params.find("username").unwrap();

    let user_wrapper = get_user(username);

    if user_wrapper.is_none() {

        redirect_to("/not-found")
    } else {

        let mut data = ViewData::new(req);
        let user = user_wrapper.unwrap();

        data.insert("cur_user", json!(user));

        respond_view("user", &data)
    }
}

pub fn change_password(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let session = get_session_obj(req);
    let username = session["username"].as_str().unwrap();
    let old_password = &params.get("oldPassword").unwrap()[0];
    let new_password = &params.get("newPassword").unwrap()[0];
    let username_wrapper = check_user_login(username, old_password);

    let mut data = JsonData::new();

    if username_wrapper.is_none() {

        data.success = false;
        data.message = "您输入的当前密码不正确！".to_owned();

        return respond_json(&data);
    }

    let result = update_password(username, new_password);

    if result.is_none() {

        data.success = false;
        data.message = "密码更改失败，请重新尝试！".to_owned();

        return respond_json(&data);
    } else {

        data.data = json!("/login");

        req.session().clear().unwrap();

        respond_json(&data)
    }
}