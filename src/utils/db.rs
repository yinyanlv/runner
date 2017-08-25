extern crate mysql;

use mysql;
use config::Config;

pub struct MySqlPool(mysql::Pool);

impl MySqlPool {

    pub fn new(config: &Config) -> MySqlPool {

        let table = config.value();
        let db = table.get("database").unwrap().as_table().unwrap();
        let user = db.get("user").unwrap().as_str().unwrap();
        let password = db.get("password").unwrap().as_str().unwrap();
        let host = db.get("host").unwrap().as_str().unwrap();
        let port = db.get("port").unwrap().as_str().unwrap();
        let db_name = db.get("db_name").unwrap().as_str().unwrap();

        let mut builder = mysql::OptsBuilder::default();

        builder.user(Some(user))
            .pass(Some(password))
            .ip_or_hostname(Some(host))
            .tcp_port(port as u8)
            .db_name(Some(db_name));

        let pool = mysql::Pool::new(build).unwrap();

        MySqlPool(pool)
    }

    pub fn value(&self) -> mysql::Pool {

        self.0.clone()
    }
}