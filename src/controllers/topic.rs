use iron::prelude::*;
use serde_json::Value;

use common::http::*;
use common::utils::*;
use common::utils::is_admin as check_is_admin;
use services::user::{get_username, get_user, get_user_id};
use services::topic::*;
use services::topic::create_topic as service_create_topic;
use services::topic::delete_topic as service_delete_topic;
use services::comment::{get_comments_by_topic_id};
use services::category::get_categories;
use services::collection::*;
use services::collection::is_collected as collection_is_collected;
use services::topic_vote::*;
use services::topic_vote::is_agreed as topic_is_agreed;
use services::topic_vote::is_disagreed as topic_is_disagreed;
use services::comment_vote::is_agreed as comment_is_agreed;
use services::comment_vote::is_disagreed as comment_is_disagreed;
use models::comment::Comment;
use models::category::Category;
use controllers::upload::sync_upload_file;

pub fn render_topic(req: &mut Request) -> IronResult<Response> {

    let is_login = is_login(req);
    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();
    let topic_wrapper = get_topic(topic_id);

    if topic_wrapper.is_none() {

        return redirect_to("/not-found");
    }

    let mut user_id_string = "".to_string();
    let mut cur_username = "".to_string();

    if is_login {  // 用户已登录
        let session = get_session_obj(req);
        let username = session["username"].as_str().unwrap();

        user_id_string = get_user_id(username).to_string();
        cur_username = username.to_string();
    }

    let user_id = &*user_id_string;

    let mut topic = topic_wrapper.unwrap();

    increment_topic_view_count(topic_id);

    let author_id = topic.user_id;
    let author_name = get_username(author_id).unwrap();
    let author = get_user(&*author_name).unwrap();

    let mut data = ViewData::new(req);

    topic.content = parse_to_html(&*topic.content);

    let comments = get_comments_by_topic_id(topic_id);
    let related_topics = get_user_other_topics(author_id, topic_id);
    let mut is_collected = false;
    let mut is_agreed = false;
    let mut is_disagreed = false;
    let mut is_admin = false;

    if user_id != "" {  // 用户已登录

        is_collected = collection_is_collected(user_id, topic_id);
        is_agreed = topic_is_agreed(user_id, topic_id);
        is_disagreed = topic_is_disagreed(user_id, topic_id);
        is_admin = check_is_admin(&*cur_username);
    }

    let list = rebuild_comments(&*author_name, user_id, is_admin, &comments);

    data.insert("title", json!(topic.title));
    data.insert("is_login", json!(is_login));
    data.insert("is_topic_page", json!(true));
    data.insert("topic", json!(topic));
    data.insert("is_admin", json!(is_admin));
    data.insert("is_user_self", json!(&*author_id.to_string() == user_id));
    data.insert("is_collected", json!(is_collected));
    data.insert("is_agreed", json!(is_agreed));
    data.insert("is_disagreed", json!(is_disagreed));
    data.insert("comments", json!(list));
    data.insert("comment_count", json!(list.len()));
    data.insert("author", json!(author));
    data.insert("related_topics", json!(related_topics));
    data.insert("is_has_related_topics", json!(related_topics.len() != 0));

    respond_view("topic", &data)
}

fn rebuild_comments(author_name: &str, user_id: &str, is_admin: bool, comments: &Vec<Comment>) -> Vec<Value> {

    let mut vec = Vec::new();
    let mut index = 0;

    if user_id == "" {  // 用户未登录

        for comment in comments.into_iter() {

            index = index + 1;

            vec.push(json!({
                "index": index,
                "comment": comment,
                "is_author": author_name == comment.username,
                "is_admin": false,
                "is_user_self": false,
                "is_highlight": comment.agree_count >= 10,
                "is_agreed": false,
                "is_disagreed": false
            }));
        }
    } else {

        for comment in comments.into_iter() {

            index = index + 1;

            vec.push(json!({
                "index": index,
                "comment": comment,
                "is_author": author_name == comment.username,
                "is_admin": is_admin,
                "is_user_self": user_id == &*comment.user_id.to_string(),
                "is_highlight": comment.agree_count >= 10,
                "is_agreed": comment_is_agreed(user_id, &*comment.id),
                "is_disagreed": comment_is_disagreed(user_id, &*comment.id)
            }));
        }
    }

    vec
}

pub fn render_create_topic(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);
    let categories = get_categories();
    let list = rebuild_categories(0, &categories);

    data.insert("title", json!("发布话题"));
    data.insert("categories", json!(list));

    respond_view("topic-editor", &data)
}

pub fn render_edit_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();

    if !is_topic_created(topic_id) {

        return redirect_to("/not-found");
    }

    let topic_wrapper = get_topic(topic_id);

    if topic_wrapper.is_none() {

        return redirect_to("/not-found");
    }

    let topic = topic_wrapper.unwrap();
    let categories = get_categories();
    let list = rebuild_categories(topic.category_id, &categories);

    let mut data = ViewData::new(req);

    data.insert("title", json!("编辑话题"));
    data.insert("topic", json!(&topic));
    data.insert("categories", json!(list));

    respond_view("topic-editor", &data)
}

fn rebuild_categories(category_id: u8, categories: &Vec<Category>) -> Vec<Value> {

    let mut vec = Vec::new();

    for category in categories.into_iter() {

        vec.push(json!({
            "category": category,
            "is_selected": category_id == category.id
        }));
    }

    vec
}

pub fn create_topic(req: &mut Request) -> IronResult<Response> {

    let session = get_session_obj(req);
    let params = get_request_body(req);
    let user_id = session["id"].as_u64().unwrap();
    let category = &params.get("category").unwrap()[0];
    let title = &params.get("title").unwrap()[0];
    let content = &params.get("content").unwrap()[0];

    let obj = json!({
        "user_id": user_id,
        "category_id": category.to_owned(),
        "title": title.to_owned(),
        "content": sync_upload_file(content)
    });

    let result = service_create_topic(&obj);

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "发布话题失败".to_string();

        return respond_json(&data);
    }

    let topic_id = result.unwrap();

    data.message = "发布话题成功".to_owned();
    data.data = json!("/topic/".to_string() + &*topic_id);

    respond_json(&data)
}

pub fn edit_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let body = get_request_body(req);
    let topic_id = params.find("topic_id").unwrap();
    let category = &body.get("category").unwrap()[0];
    let title = &body.get("title").unwrap()[0];
    let content = &body.get("content").unwrap()[0];

    let mut data = JsonData::new();

    if !is_topic_created(topic_id) {

        data.success = false;
        data.message = "未找到该话题".to_owned();

        return respond_json(&data);
    }

    let result = update_topic(topic_id, &json!({
        "category_id": category.to_owned(),
        "title": title.to_owned(),
        "content": sync_upload_file(content)
    }));

    if result.is_none() {

        data.success = false;
        data.message = "修改话题失败".to_owned();

        return respond_json(&data);
    }

    data.message = "修改话题成功".to_owned();
    data.data = json!("/topic/".to_string() + topic_id);

    respond_json(&data)
}

pub fn delete_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();

    let mut data = JsonData::new();

    if !is_topic_created(topic_id) {

        data.success = false;
        data.message = "未找到该话题".to_owned();

        return respond_json(&data);
    }

    let result = service_delete_topic(topic_id);

    if result.is_none() {

        data.success = false;
        data.message = "删除话题失败".to_owned();

        return respond_json(&data);
    }

    data.message = "删除话题成功".to_owned();
    data.data = json!("/");

    respond_json(&data)
}

pub fn collect_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();
    let body = get_request_body(req);
    let user_id = &body.get("userId").unwrap()[0];
    let is_collect = &body.get("isCollect").unwrap()[0];
    let result;

    if is_collect == "true" {

        if !is_collected(user_id, topic_id) {

            result = create_collection(user_id, topic_id);
        } else {

            result = None;
        }

    } else {

        result = delete_collection(user_id, topic_id);
    }

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "更新收藏失败".to_owned();
    }

    respond_json(&data)
}

pub fn stick_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();
    let body = get_request_body(req);
    let is_stick = &body.get("isSticked").unwrap()[0];
    let result;

    if is_stick == "true" {

        result = update_topic_sticky(topic_id, 1);
    } else {

        result = update_topic_sticky(topic_id, 0);
    }

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "更新置顶失败".to_owned();
    }

    respond_json(&data)
}

pub fn essence_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();
    let body = get_request_body(req);
    let is_essence = &body.get("isEssenced").unwrap()[0];
    let result;

    if is_essence == "true" {

        result = update_topic_essence(topic_id, 1);
    } else {

        result = update_topic_essence(topic_id, 0);
    }

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "更新精华失败".to_owned();
    }

    respond_json(&data)
}

pub fn vote_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();
    let body = get_request_body(req);
    let user_id = &body.get("userId").unwrap()[0];
    let state = &body.get("state").unwrap()[0];
    let result;

    if state == "0" {

        result = delete_topic_vote(user_id, topic_id);
    } else {

        if is_voted(user_id, topic_id) {
            result = update_topic_vote(user_id, topic_id, state);
        } else {
            result = create_topic_vote(user_id, topic_id, state);
        }
    }

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "更新失败".to_owned();
    }

    respond_json(&data)
}