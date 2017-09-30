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

pub fn increment_topic_view_count(topic_id: &str) -> Option<String> {

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE topic SET
                        view_count = view_count + 1
                        WHERE id = ?
                        "#).unwrap();
    let result = stmt.execute((
        topic_id,
    ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(topic_id.to_string())
}

pub fn update_topic(topic_id: &str, topic: &Value) -> Option<String> {

    let update_time = gen_datetime().to_string();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE topic SET
                        category_id = ?,
                        title = ?,
                        content = ?,
                        update_time = ?
                        WHERE id = ?
                        "#).unwrap();
    let result = stmt.execute((
        topic["category_id"].as_str().unwrap(),
        topic["title"].as_str().unwrap(),
        topic["content"].as_str().unwrap(),
        &*update_time,
        topic_id
    ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(topic_id.to_string())
}

pub fn delete_topic(topic_id: &str) -> Option<String> {

    let mut result = SQL_POOL.prep_exec("DELETE FROM topic where id = ?", (topic_id, ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err.message);
        return None;
    }

    Some(topic_id.to_string())
}

pub fn is_topic_created(topic_id: &str) -> bool {

    let mut result = SQL_POOL.prep_exec("SELECT count(id) from topic where id = ?", (topic_id, )).unwrap();
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

pub fn get_topic(topic_id: &str) -> Option<Topic> {

    let mut result = SQL_POOL.prep_exec(r#"
                          SELECT
                          t.id, user_id, category_id, c.name as category_name, title, content, status, priority, view_count,
                          (SELECT count(id) FROM topic_vote WHERE state = 1 AND topic_id = t.id) as agree_count,
                          (SELECT count(id) FROM topic_vote WHERE state = -1 AND topic_id = t.id) as disagree_count,
                          create_time, update_time
                          FROM topic as t
                          LEFT JOIN category as c
                          ON t.category_id = c.id
                          WHERE t.id = ?
                          "#, (topic_id, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let mut row = row_wrapper.unwrap().unwrap();

    Some(Topic {
        id: row.get::<String, _>(0).unwrap(),
        user_id: row.get::<u16, _>(1).unwrap(),
        category_id: row.get::<u8, _>(2).unwrap(),
        category_name: row.get::<String, _>(3).unwrap(),
        title: row.get::<String, _>(4).unwrap(),
        content: row.get::<String, _>(5).unwrap(),
        status: row.get::<u8, _>(6).unwrap(),
        priority: row.get::<u8, _>(7).unwrap(),
        view_count: row.get::<u32, _>(8).unwrap(),
        agree_count: row.get::<u16, _>(9).unwrap(),
        disagree_count: row.get::<u16, _>(10).unwrap(),
        create_time: row.get::<NaiveDateTime, _>(11).unwrap(),
        update_time: row.get::<NaiveDateTime, _>(12).unwrap()
    })
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