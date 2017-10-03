use iron::prelude::*;
use serde_json::Value;

use common::http::*;
use common::utils::*;
use services::topic::get_default_topic_list;
use services::comment::get_last_comment_by_topic_id;

pub fn render_home(req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);
    let has_query_params = has_request_query(req);
    let tab;
    let page: &str;

    if has_query_params {
//
//        params = get_request_query(req);
//        tab = &*params.get("tab").unwrap()[0].clone();
    } else {

        tab = "default";
    }


    let list = get_default_topic_list();

    let mut data = ViewData::new(req);

    let topic_list = rebuild_topic_list(&list);

    data.insert("has_topic_list", json!(list.len()));
    data.insert("topic_list", json!(topic_list));

    respond_view("home", &data)
}

fn rebuild_topic_list(topics: &Vec<Value>) -> Vec<Value> {

    let mut vec = Vec::new();

    for topic in topics.into_iter() {

        let topic_id = topic["topic_id"].as_str().unwrap();

        vec.push(json!({
            "topic": topic,
            "comment": get_last_comment_by_topic_id(topic_id),
        }));
    }

    vec
}