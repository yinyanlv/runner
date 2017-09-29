use iron::prelude::*;

use common::http::*;
use common::utils::*;
use services::comment::*;
use services::comment::create_comment as service_create_comment;
use services::comment::delete_comment as service_delete_comment;

pub fn create_comment(req: &mut Request) -> IronResult<Response> {

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

    let result = service_create_comment(&obj);

    let mut data = JsonData::new();

    if result.is_none() {

        data.success = false;
        data.message = "发布话题失败".to_string();

        return respond_json(&data);
    }

    let comment_id = result.unwrap();

    data.message = "发布话题成功".to_owned();
    data.data = json!("/comment/".to_string() + &*comment_id);

    respond_json(&data)
}

pub fn edit_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let body = get_request_body(req);
    let comment_id = params.find("comment_id").unwrap();
    let category = &body.get("category").unwrap()[0];
    let title = &body.get("title").unwrap()[0];
    let content = &body.get("content").unwrap()[0];

    let mut data = JsonData::new();

    if !is_comment_created(comment_id) {

        data.success = false;
        data.message = "未找到该话题".to_owned();

        return respond_json(&data);
    }

    let result = update_comment(comment_id, &json!({
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
    data.data = json!("/comment/".to_string() + comment_id);

    respond_json(&data)
}

pub fn delete_comment(req: &mut Request) -> IronResult<Response> {

    let params = get_router_params(req);
    let comment_id = params.find("comment_id").unwrap();

    let mut data = JsonData::new();

    if !is_comment_created(comment_id) {

        data.success = false;
        data.message = "未找到该话题".to_owned();

        return respond_json(&data);
    }

    let result = service_delete_comment(comment_id);

    if result.is_none() {

        data.success = false;
        data.message = "删除话题失败".to_owned();

        return respond_json(&data);
    }

    data.message = "删除话题成功".to_owned();
    data.data = json!("/");

    respond_json(&data)
}
