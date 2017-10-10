use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use mysql::Pool;
use toml::value::Table;

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

    pub static ref GITHUB_LOGIN_PATH: String = {

        let github_config = CONFIG_TABLE.get("github").unwrap().as_table().unwrap();
        let client_id = github_config.get("client_id").unwrap().as_str().unwrap();

        "https://github.com/login/oauth/authorize?client_id=".to_string() + client_id
    };
}

lazy_static! {

    pub static ref SQL_POOL: Pool = {

        MySqlPool::new(&CONFIG).value()
    };
}