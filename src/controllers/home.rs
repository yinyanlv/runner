use iron::prelude::*;
use iron::status;
use hbs::Template;
use rustc_serialize::json::{Object, Json, ToJson};

pub fn render_home(_req: &mut Request) -> IronResult<Response> {
    
    let mut res = Response::new();
    let data: Object = get_home_data();

    res.set_mut(Template::new("home", data))
        .set_mut(status::Ok);

    Ok(res)
}

fn get_home_data() -> Object {

    let mut obj = Object::new();

    obj.insert("title".to_owned(), "runner".to_owned().to_json());
    obj.insert("message".to_owned(), "hello rust, this is rennder by handlebars!".to_owned().to_json());

    obj
}