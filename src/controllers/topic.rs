use iron::prelude::*;

use common::http::*;
use common::utils::*;
use services::topic::*;
use services::topic::create_topic as service_create_topic;
use services::topic::delete_topic as service_delete_topic;
use services::comment::get_comments_by_topic_id;

pub fn render_topic(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let topic_id = params.find("topic_id").unwrap();

    let topic_wrapper = get_topic(topic_id);

    if topic_wrapper.is_none() {

        return redirect_to("/not-found");
    }

    let mut topic = topic_wrapper.unwrap();

    let mut data = ViewData::new(req);

    topic.content = parse_to_html(&*topic.content);

    let comments = get_comments_by_topic_id(topic_id);

    data.insert("is_topic_page", json!(true));
    data.insert("topic", json!(topic));
    data.insert("comments", json!(comments));

    respond_view("topic", &data)
}

pub fn render_create_topic(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("title", json!("发布话题"));
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


    let mut data = ViewData::new(req);

    data.insert("title", json!("编辑话题"));
    data.insert("topic", json!(&topic_wrapper.unwrap()));

    respond_view("topic-editor", &data)
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
        "content": content.to_owned()
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
        "content": content.to_owned()
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
