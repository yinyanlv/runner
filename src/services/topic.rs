use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;
use models::topic::Topic;

pub fn create_topic(topic: &Value) -> Option<String> {

    let create_time = gen_datetime().to_string();
    let user_id = topic["user_id"].as_u64().unwrap();
    let topic_id = gen_md5(&*(user_id.to_string() + &*create_time));

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO topic
                        (id, user_id, category_id, title, content, create_time, update_time)
                        VALUES (?, ?, ?, ?, ?, ?, ?);
                        "#).unwrap();

    let result = stmt.execute((
        &*topic_id,
        user_id,
        topic["category_id"].as_str().unwrap(),
        topic["title"].as_str().unwrap(),
        topic["content"].as_str().unwrap(),
        &*create_time,
        &*create_time
    ));

    if let Err(MySqlError(ref err)) = result {

        return None;
    }

    Some(topic_id)
}

pub fn update_topic() {

}

pub fn get_topic_count() -> u32 {

    let mut result = SQL_POOL.prep_exec("SELECT count(id) from topic", ()).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u32, )>(row);

    count
}