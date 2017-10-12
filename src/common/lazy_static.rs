use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use mysql::Pool;
use toml::value::{Table, Array};

use common::config::Config;
use common::db::MySqlPool;

pub static RECORDS_COUNT_PER_PAGE: u32 = 15;

lazy_static! {
    pub static ref HTTPS_CLIENT: Client = {

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        Client::with_connector(connector)
    };
}

lazy_static! {

    pub static ref CONFIG: Config = {

        Config::new("config.toml")
    };

    pub static ref CONFIG_TABLE: Table = {

        CONFIG.value()
    };

    pub static ref PATH: &'static str = {
        CONFIG_TABLE.get("path").unwrap().as_str().unwrap()
    };

    pub static ref STATIC_PATH: &'static str = {
        CONFIG_TABLE.get("static_path").unwrap().as_str().unwrap()
    };

    pub static ref UPLOAD_PATH: &'static str = {
        CONFIG_TABLE.get("upload_path").unwrap().as_str().unwrap()
    };

    pub static ref GITHUB_LOGIN_PATH: String = {

        let github_config = CONFIG_TABLE.get("github").unwrap().as_table().unwrap();
        let client_id = github_config.get("client_id").unwrap().as_str().unwrap();

        "https://github.com/login/oauth/authorize?client_id=".to_string() + client_id
    };

    pub static ref ADMINS: &'static Array = {

        CONFIG_TABLE.get("admins").unwrap().as_array().unwrap()
    };

    pub static ref SESSION_KEY: &'static str = {

        let redis_config = CONFIG_TABLE.get("redis").unwrap().as_table().unwrap();
        redis_config.get("session_key").unwrap().as_str().unwrap()
    };

    pub static ref UPLOAD_TEMP_PATH: &'static str = {

        let upload_config = CONFIG_TABLE.get("upload").unwrap().as_table().unwrap();
        upload_config.get("temp_path").unwrap().as_str().unwrap()
    };

    pub static ref UPLOAD_ASSETS_PATH: &'static str = {

        let upload_config = CONFIG_TABLE.get("upload").unwrap().as_table().unwrap();
        upload_config.get("assets_path").unwrap().as_str().unwrap()
    };
}

lazy_static! {

    pub static ref SQL_POOL: Pool = {

        MySqlPool::new(&CONFIG).value()
    };
}