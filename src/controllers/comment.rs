use iron::prelude::*;

use common::http::*;
use common::utils::*;
use services::comment::*;
use services::comment::create_comment as service_create_comment;
use services::comment::delete_comment as service_delete_comment;

pub fn create_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let user_id = &params.get("userId").unwrap()[0];
    let topic_id = &params.get("topicId").unwrap()[0];
    let content = &params.get("content").unwrap()[0];

    let obj = json!({
        "user_id": user_id.to_owned(),
        "topic_id": topic_id.to_owned(),
        "content": content.to_owned()
    });

    let result = service_create_comment(&obj);

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "回复失败".to_string();

        return respond_json(&data);
    }

    let comment_id = result.unwrap();

    data.message = "发布话题成功".to_owned();
    data.data = json!("/topic/".to_string() + topic_id);

    respond_json(&data)
}

pub fn render_edit_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let comment_id = params.find("comment_id").unwrap();

    if !is_comment_created(comment_id) {

        return redirect_to("/not-found");
    }

    let content_wrapper = get_comment_content(comment_id);

    if content_wrapper.is_none() {

        return redirect_to("/not-found");
    }

    let content = content_wrapper.unwrap();

    let mut data = ViewData::new(req);

    data.insert("comment_id", json!(comment_id.to_string()));
    data.insert("content", json!(content));

    respond_view("comment-editor", &data)
}

pub fn edit_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let body = get_request_body(req);
    let comment_id = params.find("comment_id").unwrap();
    let content = &body.get("content").unwrap()[0];

    let mut data = JsonData::new();

    if !is_comment_created(comment_id) {

        data.success = false;
        data.message = "未找到该回复".to_owned();

        return respond_json(&data);
    }

    let result = update_comment(comment_id, &json!({
        "comment_id": comment_id.to_owned(),
        "content": content.to_owned()
    }));

    if result.is_none() {

        data.success = false;
        data.message = "修改回复失败".to_owned();

        return respond_json(&data);
    }

    let topic_id = &*get_comment(comment_id).unwrap().topic_id;

    data.message = "修改回复成功".to_owned();
    data.data = json!("/topic/".to_string() + topic_id);

    respond_json(&data)
}

pub fn delete_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let comment_id = params.find("comment_id").unwrap();
    let body = get_request_body(req);
    let topic_id = &body.get("topicId").unwrap()[0];

    let mut data = JsonData::new();

    if !is_comment_created(comment_id) {

        data.success = false;
        data.message = "未找到该回复".to_owned();

        return respond_json(&data);
    }

    let result = service_delete_comment(comment_id);

    if result.is_none() {

        data.success = false;
        data.message = "删除回复失败".to_owned();

        return respond_json(&data);
    }

    data.message = "删除回复成功".to_owned();
    data.data = json!("/topic/".to_owned() + topic_id);

    respond_json(&data)
}


pub fn vote_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let comment_id = params.find("comment_id").unwrap();
    let body = get_request_body(req);
    let topic_id = &body.get("topicId").unwrap()[0];

    let mut data = JsonData::new();

    if !is_comment_created(comment_id) {

        data.success = false;
        data.message = "未找到该回复".to_owned();

        return respond_json(&data);
    }

    let result = service_delete_comment(comment_id);

    if result.is_none() {

        data.success = false;
        data.message = "删除回复失败".to_owned();

        return respond_json(&data);
    }

    data.message = "删除回复成功".to_owned();
    data.data = json!("/topic/".to_owned() + topic_id);

    respond_json(&data)
}
