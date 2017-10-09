extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate iron_sessionstorage2 as iron_sessionstorage;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate urlencoded;
extern crate chrono;
extern crate crypto;
extern crate rand;
extern crate toml;
extern crate hyper;
extern crate hyper_native_tls;
extern crate url;
extern crate multipart;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate mime;
#[macro_use]
extern crate mysql;
extern crate pulldown_cmark;
extern crate regex;
extern crate rss;

mod common;
mod routes;
mod controllers;
mod services;
mod models;

use std::path::Path;

use iron::Chain;
use mount::Mount;
use staticfile::Static;
use hbs::{HandlebarsEngine, DirectorySource};
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::RedisBackend;

use common::lazy_static::CONFIG;
use common::db::get_redis_config;
use common::middlewares::FlowControl;
use common::utils::mount_template_var;

fn main() {

    let mut chain = Chain::new(routes::gen_router());

    chain.link_before(FlowControl);

    let mut hbs_engine = HandlebarsEngine::new();
    hbs_engine.add(Box::new(DirectorySource::new("views/", ".hbs")));
    hbs_engine.handlebars_mut().register_helper("var", Box::new(mount_template_var));
    hbs_engine.reload().unwrap();
    chain.link_after(hbs_engine);

    let redis_config = &*get_redis_config(&CONFIG);
    chain.link_around(SessionStorage::new(RedisBackend::new(redis_config).unwrap()));

    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount("static/", Static::new(Path::new("static")));

    let host = CONFIG.get("host").as_str().unwrap();
    let port: &str = &*CONFIG.get("port").as_integer().unwrap().to_string();

    println!("http server is listening on port {}!", port);
    iron::Iron::new(mount)
        .http(host.to_string() + ":" + port)
        .unwrap();
}