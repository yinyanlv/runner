extern crate serde_json;

use iron::prelude::*;
use iron::status;
use hbs::Template;
use hbs::handlebars::to_json;
use self::serde_json::value::{Value, Map};

pub fn render_home(_req: &mut Request) -> IronResult<Response> {
    
    let mut res = Response::new();
    let data = get_home_data();

    res.set_mut(Template::new("home", data))
        .set_mut(status::Ok);

    Ok(res)
}

fn get_home_data() ->  Map<String, Value> {

    let mut data = Map::new();

    data.insert("title".to_owned(), to_json(&"runner".to_owned()));
    data.insert("message".to_owned(), to_json(&"hello rust, this is rennder by handlebars!".to_owned()));

    data
}

