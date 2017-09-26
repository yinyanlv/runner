use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use mysql::Pool;
use toml::value::Table;

use common::config::Config;
use common::db::MySqlPool;

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
}

lazy_static! {

    pub static ref SQL_POOL: Pool = {

        MySqlPool::new(&CONFIG).value()
    };
}