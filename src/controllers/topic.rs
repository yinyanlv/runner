use iron::prelude::*;

use common::http::*;
use common::utils::*;
use services::topic::create_topic as service_create_topic;

pub fn render_topic(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("is_topic_page", json!(true));

    respond_view("topic", &data)
}

pub fn render_create_topic(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("title", json!("发布话题"));
    respond_view("topic-editor", &data)
}

pub fn render_edit_topic(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);

    data.insert("title", json!("编辑话题"));
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

    data.data = json!("/topic/".to_string() + &*topic_id);

    respond_json(&data)
}

pub fn edit_topic(req: &mut Request) -> IronResult<Response> {

    let session = get_session_obj(req);
    let body = get_request_body(req);
    let username = session["username"].as_str().unwrap();

    let data = JsonData::new();

    respond_json(&data)
}

pub fn delete_topic(req: &mut Request) -> IronResult<Response> {

    let data = JsonData::new();

    respond_json(&data)
}
