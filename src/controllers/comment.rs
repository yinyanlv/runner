use iron::prelude::*;
use regex::{Regex, Captures};

use common::http::*;
use common::utils::*;
use services::comment::*;
use services::comment::create_comment as service_create_comment;
use services::comment::delete_comment as service_delete_comment;
use services::comment_vote::*;
use services::user::get_user_id;
use services::topic::get_topic;
use services::message::create_message;
use controllers::upload::sync_upload_file;

pub fn create_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_request_body(req);
    let user_id = &params.get("userId").unwrap()[0];
    let topic_id = &params.get("topicId").unwrap()[0];
    let content = &params.get("content").unwrap()[0];

    let reg = Regex::new(r"\B@([\da-zA-Z_]+)").unwrap();
    let mut mentions: Vec<u16> = Vec::new();
    let new_content = reg.replace_all(&content, |caps: &Captures| {

        let username = caps.get(1).unwrap().as_str();
        let user_id = get_user_id(username);

        if user_id == 0 {

            format!("@{}", username)
        } else {
            mentions.push(user_id);

            format!("[@{}]({}{})", username, "/user/", username)
        }
    });

    let obj = json!({
        "user_id": user_id.to_owned(),
        "topic_id": topic_id.to_owned(),
        "content": sync_upload_file(&*new_content.to_string())
    });

    let result = service_create_comment(&obj);

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "回复失败".to_string();

        return respond_json(&data);
    }

    let comment_id = result.unwrap();
    let topic = get_topic(topic_id).unwrap();

    if topic.user_id != user_id.parse::<u16>().unwrap() {  // 忽略作者自己的回复
        create_message(&json!({
            "comment_id": comment_id,
            "topic_id": topic_id.to_owned(),
            "from_user_id": user_id.to_owned(),
            "to_user_id": topic.user_id,
            "type": 0
        }));
    }

    mentions.dedup();

    for mention in mentions.iter().filter(|&id| *id != topic.user_id && *id != user_id.parse::<u16>().unwrap()) {  // 忽略@作者或自己

        create_message(&json!({
            "comment_id": comment_id,
            "topic_id": topic_id.to_owned(),
            "from_user_id": user_id.to_owned(),
            "to_user_id": mention,
            "type": 1
        }));
    }

    data.message = "发表评论成功".to_owned();
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
        "content": sync_upload_file(content)
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
    let user_id = &body.get("userId").unwrap()[0];
    let state = &body.get("state").unwrap()[0];
    let result;

    if state == "0" {

        result = delete_comment_vote(user_id, comment_id);
    } else {

        if is_voted(user_id, comment_id) {
            result = update_comment_vote(user_id, comment_id, state);
        } else {
            result = create_comment_vote(user_id, comment_id, state);
        }
    }

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "更新失败".to_owned();
    }

    respond_json(&data)
}
