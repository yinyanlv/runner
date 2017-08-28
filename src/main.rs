extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate persistent;

mod route;
mod controllers;
mod utils;

use std::path::Path;

use iron::Chain;
use mount::Mount;
use persistent::Read;
use hbs::{HandlebarsEngine, DirectorySource};
use staticfile::Static;

use utils::config::Config;
use utils::db::MySqlPool;

fn main() {

    let mut chain = Chain::new(route::get_router());

    let config = Config::new("config.toml");
    chain.link_before(Read::<Config>::one(config.clone()));
   
    let sql_pool = MySqlPool::new(&config);
    chain.link_before(Read::<MySqlPool>::one(sql_pool));


    let mut hbs_engine = HandlebarsEngine::new();
    hbs_engine.add(Box::new(DirectorySource::new("/templates/", ".hbs")));
    hbs_engine.reload().unwrap();
    chain.link_after(hbs_engine);

    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount("/static/", Static::new(Path::new("static")));

    let host = config.get("host").as_str().unwrap();
    let port = config.get("port").as_integer().unwrap();

    println!("http server is listenning on port {}", port);
    iron::Iron::new(mount)
        .http(host.to_string() + &*(port.to_string()))
        .unwrap();
}