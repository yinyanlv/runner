use mysql;

use iron::typemap::Key;

use common::config::Config;

pub struct MySqlPool(mysql::Pool);

impl MySqlPool {

    pub fn new(config: &Config) -> MySqlPool {

        let table = config.value();
        let mysql_config = table.get("mysql").unwrap().as_table().unwrap();
        let username = mysql_config.get("username").unwrap().as_str().unwrap();
        let password = mysql_config.get("password").unwrap().as_str().unwrap();
        let host = mysql_config.get("host").unwrap().as_str().unwrap();
        let port = mysql_config.get("port").unwrap().as_integer().unwrap();
        let db_name = mysql_config.get("db_name").unwrap().as_str().unwrap();

        let mut builder = mysql::OptsBuilder::default();

        builder.user(Some(username))
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

pub struct RedisConfig {
    pub connect_string: String,
    pub expire: u64
}

pub fn get_redis_config(config: &Config) -> RedisConfig {

    let table = config.value();
    let redis_config = table.get("redis").unwrap().as_table().unwrap();
    let protocol = redis_config.get("protocol").unwrap().as_str().unwrap();
    let host = redis_config.get("host").unwrap().as_str().unwrap();
    let port = redis_config.get("port").unwrap().as_integer().unwrap().to_string();
    let username = redis_config.get("username").unwrap().as_str().unwrap();
    let password = redis_config.get("password").unwrap().as_str().unwrap();
    let max_age = redis_config.get("max_age").unwrap().as_integer().unwrap();
    let connect_string;

    if password == "" {

        connect_string = format!("{}://{}:{}", protocol, host, &*port)
    } else {

        connect_string = format!("{}://{}:{}@{}:{}", protocol, username, password, host, &*port)
    }

    RedisConfig {
        connect_string: connect_string,
        expire: max_age as u64
    }
}