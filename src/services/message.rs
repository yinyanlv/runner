use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;

use common::utils::*;
use common::lazy_static::{SQL_POOL, RECORDS_COUNT_PER_PAGE};

pub fn create_message(message: &Value) -> Option<String> {

    let create_time = gen_datetime().to_string();
    let comment_id = message["comment_id"].as_str().unwrap();
    let message_id = gen_md5(&*(comment_id.to_string() + &*create_time));

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO message
                        (id, comment_id, topic_id, from_user_id, to_user_id, type, create_time)
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

pub fn get_user_message_list(user_id: u16, page: u32) -> Vec<Value> {

    let offset;

    if page <= 1 {
        offset = 0;
    } else {
        offset = (page - 1) * RECORDS_COUNT_PER_PAGE;
    }

    let sql = r#"
        SELECT
        m.id as message_id, m.comment_id, m.topic_id, t.title, u.username, m.type
        FROM message AS m
        LEFT JOIN topic AS t
        ON m.topic_id = t.id
        LEFT JOIN user AS u
        ON m.from_user_id = u.id
        WHERE m.to_user_id = ?
        ORDER BY m.create_time DESC
        LIMIT ? OFFSET ?
        "#;

    let result = SQL_POOL.prep_exec(sql, (user_id, RECORDS_COUNT_PER_PAGE, offset)).unwrap();

    result.map(|row_wrapper| row_wrapper.unwrap())
        .map(|mut row| {
            json!({
                "message_id": row.get::<String, _>(0).unwrap(),
                "comment_id": row.get::<String, _>(1).unwrap(),
                "topic_id": row.get::<String, _>(2).unwrap(),
                "title": row.get::<String, _>(3).unwrap(),
                "username": row.get::<String, _>(4).unwrap(),
                "type": row.get::<u8, _>(5).unwrap()
            })
        })
        .collect()
}

pub fn get_user_message_list_count(user_id: u16) -> u32 {

    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM message WHERE to_user_id = ?", (user_id, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u32, )>(row);

    count
}

pub fn delete_message(message_id: &str) -> Option<String> {

    let result = SQL_POOL.prep_exec("DELETE FROM message WHERE id = ?", (message_id, ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(message_id.to_string())
}

pub fn delete_all_message_by_user_id(user_id: u16) -> Option<u8> {

    let result = SQL_POOL.prep_exec("DELETE FROM message WHERE to_user_id = ?", (user_id, ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}