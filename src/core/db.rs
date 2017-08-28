#![allow(dead_code)]

extern crate mysql;

use iron::typemap::Key;

use core::config::Config;

pub struct MySqlPool(mysql::Pool);

impl MySqlPool {

    pub fn new(config: &Config) -> MySqlPool {

        let table = config.value();
        let db = table.get("database").unwrap().as_table().unwrap();
        let user = db.get("user").unwrap().as_str().unwrap();
        let password = db.get("password").unwrap().as_str().unwrap();
        let host = db.get("host").unwrap().as_str().unwrap();
        let port = db.get("port").unwrap().as_integer().unwrap();
        let db_name = db.get("db_name").unwrap().as_str().unwrap();

        let mut builder = mysql::OptsBuilder::default();

        builder.user(Some(user))
            .pass(Some(password))
            .ip_or_hostname(Some(host))
            .tcp_port(port as u16)
            .db_name(Some(db_name))
            .prefer_socket(false);  // 默认为true，为true时win10报错

        let pool = mysql::Pool::new(builder).unwrap();

        MySqlPool(pool)
    }

    pub fn value(&self) -> mysql::Pool {

        self.0.clone()
    }
}

impl Key for MySqlPool {

    type Value = MySqlPool;
}