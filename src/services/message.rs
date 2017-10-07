use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;

pub fn create_message(message: &Value) -> Option<String> {

    let create_time = gen_datetime().to_string();
    let comment_id = message["comment_id"].as_str().unwrap();
    let message_id = gen_md5(&*(comment_id.to_string() + &*create_time));

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO message
                        (id, topic_id, comment_id, from_user_id, to_user_id, type, create_time)
                        VALUES (?, ?, ?, ?, ?, ?, ?);
                        "#).unwrap();

    let result = stmt.execute((
        &*message_id,
        comment_id,
        message["topic_id"].as_str().unwrap(),
        message["from_user_id"].as_str().unwrap(),
        message["to_user_id"].as_u64().unwrap(),
        message["type"].as_u64().unwrap(),
        &*create_time
    ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err);
        return None;
    }

    Some(message_id)
}