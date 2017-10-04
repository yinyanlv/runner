use iron::prelude::*;
use serde_json::Value;

use common::http::*;
use common::utils::*;
use services::topic::*;
use services::comment::get_last_comment_by_topic_id;

pub fn render_default_topic_list(req: &mut Request) -> IronResult<Response> {

    render_topic_list("default", req)
}

pub fn render_essence_topic_list(req: &mut Request) -> IronResult<Response> {

    render_topic_list("essence", req)
}

pub fn render_latest_topic_list(req: &mut Request) -> IronResult<Response> {

    render_topic_list("latest", req)
}

pub fn render_no_reply_topic_list(req: &mut Request) -> IronResult<Response> {

    render_topic_list("no-reply", req)
}

pub fn render_ask_topic_list(req: &mut Request) -> IronResult<Response> {

    render_topic_list("ask", req)
}

pub fn render_share_topic_list(req: &mut Request) -> IronResult<Response> {

    render_topic_list("share", req)
}

pub fn render_job_topic_list(req: &mut Request) -> IronResult<Response> {

    render_topic_list("job", req)
}

fn render_topic_list(tab_code: &str, req: &mut Request) -> IronResult<Response> {

    let mut data = ViewData::new(req);
    let page: u32 = get_query_page(req);
    let mut data = ViewData::new(req);
    let base_url;

    match tab_code {
        "essence" => {
            data.insert("title", json!("首页-精华"));
            data.insert("is_essence_active", json!(true));
            base_url = "/topics/essence?page=";
        }
        "latest" => {
            data.insert("title", json!("首页-最新"));
            data.insert("is_latest_active", json!(true));
            base_url = "/topics/latest?page=";
        }
        "no-reply" => {
            data.insert("title", json!("首页-等待回复"));
            data.insert("is_no_reply_active", json!(true));
            base_url = "/topics/no-reply?page=";
        }
        "ask" => {
            data.insert("title", json!("首页-问答"));
            data.insert("is_ask_active", json!(true));
            base_url = "/topics/ask?page=";
        }
        "share" => {
            data.insert("title", json!("首页-分享"));
            data.insert("is_share_active", json!(true));
            base_url = "/topics/share?page=";
        }
        "job" => {
            data.insert("title", json!("首页-招聘"));
            data.insert("is_job_active", json!(true));
            base_url = "/topics/job?page=";
        }
        _ => {
            data.insert("title", json!("首页"));
            data.insert("is_default_active", json!(true));
            base_url = "/?page=";
        }
    }

    let list = get_topic_list(tab_code, page);
    let default_list_count = get_topic_list_count(tab_code);

    let topic_list = rebuild_topic_list(&list);
    let pagination = build_pagination(page, default_list_count, base_url);

    data.insert("has_topic_list", json!(topic_list.len()));
    data.insert("topic_list", json!(topic_list));
    data.insert("pagination", json!(pagination));

    respond_view("topic_list", &data)
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