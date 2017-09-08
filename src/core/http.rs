use iron::prelude::*;
use iron::status;
use iron::Url;
use iron::modifiers::Redirect;
use hbs::Template;
use hbs::handlebars::{to_json};
use persistent::Read;
use serde_json::to_string;
use serde_json::value::{Map, Value};

use core::config::Config;

pub struct ResponseData(Map<String, Value>);

impl ResponseData {

    pub fn new(req: &mut Request) -> ResponseData {

        let config = req.get::<Read<Config>>().unwrap().value();
        let path = config.get("path");
        let static_path = config.get("static_path");

        let mut map = Map::new();
        map.insert("path".to_owned(), to_json(&path.to_owned()));
        map.insert("static_path".to_owned(), to_json(&static_path.to_owned()));

        ResponseData(map)
    }

    pub fn insert(&mut self, key: &str, value: Value) -> &mut Self {

        self.0.insert(key.to_owned(), value);
        self
    }
}

pub fn respond_view(template_path: &str, data: &ResponseData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(Template::new(template_path, data.0.clone()));

    Ok(res)
}

pub fn respond_json(data: &ResponseData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(mime!(Application/Json))
        .set_mut(to_string(&data.0).unwrap());

    Ok(res)
}

pub fn redirect_to(url: &str) -> IronResult<Response> {

    let url = Url::parse(url).unwrap();
    let res = Response::with((status::Found, Redirect(url)));

    return Ok(res);
}

