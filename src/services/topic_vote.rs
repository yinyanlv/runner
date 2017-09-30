use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;

pub fn is_voted(user_id: &str, topic_id: &str) -> bool {

    let mut result = SQL_POOL.prep_exec(r#"
                        SELECT count(id) FROM topic_vote
                        WHERE
                        user_id = ? AND topic_id = ?
                        "#, (user_id, topic_id)).unwrap();

    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return false;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (count, ) = from_row::<(u8, )>(row);

    if count == 0 {
        false
    } else {
        true
    }
}

pub fn create_topic_vote(user_id: &str, topic_id: &str, state: &str) -> Option<u8> {

    let create_time = gen_datetime().to_string();
    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO topic_vote
                        (user_id, topic_id, state, create_time, update_time)
                        VALUES
                        (?, ?, ?, ?)
                        "#).unwrap();

    let result = stmt.execute((user_id, topic_id, state, &*create_time, &*create_time));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}


pub fn update_topic_vote(user_id: &str, topic_id: &str, state: &str) -> Option<u8> {

    let update_time = gen_datetime().to_string();
    let stmt = SQL_POOL.prepare(r#"
                        UPDATE topic_vote SET
                        user_id = ?,
                        topic_id = ?,
                        state = ?,
                        update_time = ?
                        "#).unwrap();

    let result = stmt.execute((user_id, topic_id, state, &*update_time));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}

pub fn delete_topic_vote(user_id: &str, topic_id: &str) -> Option<u8> {

    let mut stmt = SQL_POOL.prepare(r#"
                        DELETE FROM topic_vote
                        WHERE
                        user_id = ? AND topic_id = ?
                        "#).unwrap();

    let result = stmt.execute((user_id, topic_id));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}