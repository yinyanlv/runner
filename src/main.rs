extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate persistent;
extern crate serde_json;
extern crate urlencoded;
extern crate chrono;  // 日期时间
extern crate crypto;
extern crate rand;

#[macro_use]
extern crate mysql;

mod core;
mod route;
mod controllers;

use std::path::Path;

use iron::Chain;
use mount::Mount;
use persistent::Read;
use hbs::{HandlebarsEngine, DirectorySource};
use staticfile::Static;

use core::config::Config;
use core::db::MySqlPool;

fn main() {

    let mut chain = Chain::new(route::gen_router());

    let config = Config::new("config.toml");
    chain.link_before(Read::<Config>::one(config.clone()));
   
    let sql_pool = MySqlPool::new(&config);
    chain.link_before(Read::<MySqlPool>::one(sql_pool));

    let mut hbs_engine = HandlebarsEngine::new();
    hbs_engine.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    hbs_engine.reload().unwrap();
    chain.link_after(hbs_engine);

    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount("static/", Static::new(Path::new("static")));

    let host = config.get("host").as_str().unwrap();
    let port: &str = &*config.get("port").as_integer().unwrap().to_string();

    println!("http server is listenning on port {}", port);
    iron::Iron::new(mount)
        .http(host.to_string() + ":" + port)
        .unwrap();
}