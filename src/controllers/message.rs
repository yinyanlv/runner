use iron::prelude::*;

use common::http::*;
use common::utils::*;
use services::message::*;
use services::user::get_user_id;

pub fn render_unread_message(req: &mut Request) -> IronResult<Response> {

    let session = get_session_obj(req);
    let username = session["username"].as_str().unwrap();
    let user_id = get_user_id(username);
    let page: u32 = get_query_page(req);
    let base_url = "/".to_string() + username + "/message/unread?page=";

    let list = get_user_message_list(user_id, page);
    let list_count = get_user_message_list_count(user_id);

    let pagination = build_pagination(page, list_count, &*base_url);

    let mut data = ViewData::new(req);

    data.insert("has_message_list", json!(list.len()));
    data.insert("message_list", json!(list));
    data.insert("pagination", json!(pagination));

    respond_view("message", &data)
}

pub fn read_message(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let message_id = params.find("message_id").unwrap();
    let query = get_request_query(req);
    let topic_id = &*query.get("topic-id").unwrap()[0];
    let comment_id = &*query.get("comment-id").unwrap()[0];

    delete_message(message_id);

    let url = "/topic/".to_string() + topic_id + "#" + comment_id;
    redirect_to(&*url)
}
