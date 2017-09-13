extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
extern crate iron_sessionstorage;
extern crate persistent;
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
#[macro_use]
extern crate mime;
#[macro_use]
extern crate mysql;

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
use persistent::Read;

use common::config::Config;
use common::db::{MySqlPool, get_redis_config};
use common::middleware::GlobalControl;

fn main() {

    let mut chain = Chain::new(routes::gen_router());

    let config = Config::new("config.toml");
    chain.link_before(Read::<Config>::one(config.clone()));

    let sql_pool = MySqlPool::new(&config);
    chain.link_before(Read::<MySqlPool>::one(sql_pool));

    let mut hbs_engine = HandlebarsEngine::new();
    hbs_engine.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    hbs_engine.reload().unwrap();
    chain.link_after(hbs_engine);

    let redis_config = &*get_redis_config(&config);
    chain.link_around(SessionStorage::new(RedisBackend::new(redis_config).unwrap()));

    chain.link_before(GlobalControl);
    chain.link_after(GlobalControl);

    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount("static/", Static::new(Path::new("static")));

    let host = config.get("host").as_str().unwrap();
    let port: &str = &*config.get("port").as_integer().unwrap().to_string();

    println!("http server is listening on port {}!", port);
    iron::Iron::new(mount)
        .http(host.to_string() + ":" + port)
        .unwrap();
}