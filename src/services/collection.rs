use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;

pub fn update_collection(user_id: &str, topic_id: &str, is_collect: &str) -> Option<u8> {

    let create_time = gen_datetime().to_string();
    let mut stmt;
    let result;

    if is_collect == "true" {

        stmt = SQL_POOL.prepare(r#"
                        INSERT INTO collection
                        (user_id, topic_id, create_time)
                        VALUES
                        (?, ?, ?)
                        "#).unwrap();
        result = stmt.execute((
            user_id,
            topic_id,
            &*create_time,
        ));
    } else {

        stmt = SQL_POOL.prepare(r#"
                        DELETE FROM collection
                        WHERE
                        user_id = ? AND topic_id = ?
                        "#).unwrap();

        result = stmt.execute((user_id, topic_id));
    }


    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}
