use iron::prelude::*;
use iron::status;
use iron::Url;
use iron::modifiers::Redirect;
use hbs::Template;
use serde_json::value::{Map, Value};
use iron_sessionstorage::Value as SessionValue;
use iron_sessionstorage::traits::SessionRequestExt;

use common::utils::*;
use common::lazy_static::CONFIG_TABLE;
use services::user::get_user_count;
use services::topic::get_topic_count;
use services::comment::get_comment_count;
use services::message::get_user_message_list_count;

#[derive(Debug, Clone)]
pub struct SessionData {
    pub user: String
}

impl SessionValue for SessionData {

    fn get_key() -> &'static str {

        "runner"
    }

    fn into_raw(self) -> String {

        self.user
    }

    fn from_raw(value: String) -> Option<SessionData> {

        if value.is_empty() {

            None
        } else {

            Some(SessionData {
                user: value
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewData(Map<String, Value>);

impl ViewData {

    pub fn new(req: &mut Request) -> ViewData {

        let path = CONFIG_TABLE.get("path").unwrap().as_str().unwrap();
        let static_path = CONFIG_TABLE.get("static_path").unwrap().as_str().unwrap();
        let session_wrapper = req.session().get::<SessionData>().unwrap();

        let mut map = Map::new();
        map.insert("path".to_owned(), json!(&path));
        map.insert("static_path".to_owned(), json!(&static_path));
        map.insert("user_count".to_owned(), json!(get_user_count()));
        map.insert("topic_count".to_owned(), json!(get_topic_count()));
        map.insert("comment_count".to_owned(), json!(get_comment_count()));

        if session_wrapper.is_some() {
            let session_obj = json_parse(&*session_wrapper.unwrap().into_raw());
            let user_id = session_obj["id"].as_u64().unwrap();

            map.insert("user".to_owned(), session_obj);

            let unread_message_count = get_user_message_list_count(user_id as u16);

            map.insert("has_unread_message".to_owned(), json!(unread_message_count));
        }

        ViewData(map)
    }

    pub fn insert(&mut self, key: &str, value: Value) -> &mut Self {

        self.0.insert(key.to_owned(), value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonData {
    pub success: bool,
    pub message: String,
    pub data: Value
}

impl JsonData {

    pub fn new() -> JsonData {

        JsonData {
            success: true,
            message: "".to_owned(),
            data: json!("")
        }
    }
}

pub fn respond_view(template_path: &str, data: &ViewData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(Template::new(template_path, data.0.clone()));

    Ok(res)
}

pub fn respond_unauthorized_json(data: &JsonData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Unauthorized)
        .set_mut(mime!(Application/Json))
        .set_mut(json_stringify(data));

    Ok(res)
}

pub fn respond_forbidden_json(data: &JsonData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Forbidden)
        .set_mut(mime!(Application/Json))
        .set_mut(json_stringify(data));

    Ok(res)
}

pub fn respond_json(data: &JsonData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(mime!(Application/Json))
        .set_mut(json_stringify(data));

    Ok(res)
}

pub fn response_text(text: &str) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(mime!(Text/Plain))
        .set_mut(text.to_string());

    Ok(res)
}

pub fn redirect_to(url: &str) -> IronResult<Response> {

    let path = CONFIG_TABLE.get("path").unwrap().as_str().unwrap();
    let complete_url = path.to_string() + url;

    let url = Url::parse(&*complete_url).unwrap();
    let res = Response::with((status::Found, Redirect(url)));

    return Ok(res);
}

