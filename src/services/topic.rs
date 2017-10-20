use mysql::from_row;
use mysql::error::Error::MySqlError;
use mysql::QueryResult;
use serde_json::Value;
use chrono::{NaiveDateTime, DateTime, Local, Offset};
use rss::{Item, Guid};

use common::utils::*;
use common::lazy_static::{SQL_POOL, RECORDS_COUNT_PER_PAGE, PATH};
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

        println!("{:?}", err.message);
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

pub fn update_topic_sticky(topic_id: &str, state: u8) -> Option<String> {
    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE topic SET
                        sticky = ?
                        WHERE id = ?
                        "#).unwrap();
    let result = stmt.execute((state, topic_id));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(topic_id.to_string())
}

pub fn update_topic_essence(topic_id: &str, state: u8) -> Option<String> {
    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE topic SET
                        essence = ?
                        WHERE id = ?
                        "#).unwrap();
    let result = stmt.execute((state, topic_id));

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
    let result = SQL_POOL.prep_exec("DELETE FROM topic WHERE id = ?", (topic_id, ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(topic_id.to_string())
}

pub fn is_topic_created(topic_id: &str) -> bool {
    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM topic WHERE id = ?", (topic_id, )).unwrap();
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
                          t.id, user_id, category_id, c.name AS category_name, title, content, status, sticky, essence, view_count,
                          (SELECT count(id) FROM topic_vote WHERE state = 1 AND topic_id = t.id) AS agree_count,
                          (SELECT count(id) FROM topic_vote WHERE state = -1 AND topic_id = t.id) AS disagree_count,
                          create_time, update_time
                          FROM topic AS t
                          LEFT JOIN category AS c
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
        sticky: row.get::<u8, _>(7).unwrap(),
        essence: row.get::<u8, _>(8).unwrap(),
        view_count: row.get::<u32, _>(9).unwrap(),
        agree_count: row.get::<u16, _>(10).unwrap(),
        disagree_count: row.get::<u16, _>(11).unwrap(),
        create_time: row.get::<NaiveDateTime, _>(12).unwrap(),
        update_time: row.get::<NaiveDateTime, _>(13).unwrap()
    })
}

pub fn get_user_other_topics(user_id: u16, topic_id: &str) -> Vec<Value> {
    let result = SQL_POOL.prep_exec(r#"
                                SELECT id, title FROM topic WHERE user_id = ? AND id != ? ORDER BY create_time LIMIT 5
                                "#, (user_id, topic_id)).unwrap();

    result.map(|row_wrapper| row_wrapper.unwrap())
        .map(|mut row| {
            json!({
                "id": row.get::<String, _>(0).unwrap(),
                "title": row.get::<String, _>(1).unwrap(),
            })
        })
        .collect()
}

pub fn get_topic_count() -> u32 {
    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM topic", ()).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u32, )>(row);

    count
}

pub fn get_topic_list(tab_code: &str, page: u32) -> Vec<Value> {
    let offset;
    let sql_tpl_1 = r#"
                  SELECT
                  t.id AS topic_id, user_id AS author_id, username AS author_name, avatar_url AS author_avatar_url, c.name AS category_name, title, view_count, t.create_time,
                  (SELECT count(comment.id) FROM comment WHERE comment.topic_id = t.id) AS comment_count,
                  t.sticky AS topic_sticky,
                  t.essence AS topic_essence,
                  (SELECT create_time FROM comment WHERE topic_id = t.id ORDER BY create_time DESC LIMIT 1 OFFSET 0) AS last_comment_time
                  FROM topic AS t
                  LEFT JOIN category AS c
                  ON t.category_id = c.id
                  LEFT JOIN user AS u
                  ON t.user_id = u.id"#.to_string();
    let sql_tpl_2 = r#"
                  ORDER BY t.sticky DESC, last_comment_time DESC, t.create_time DESC
                  LIMIT ? OFFSET ?
                  "#;
    let sql;

    if page <= 1 {
        offset = 0;
    } else {
        offset = (page - 1) * RECORDS_COUNT_PER_PAGE;
    }

    match tab_code {
        "essence" => {
            sql = sql_tpl_1 + " WHERE t.essence = 1 " + sql_tpl_2;
        }
        "latest" => {
            sql = sql_tpl_1 + r#" ORDER BY t.create_time DESC
                                 LIMIT ? OFFSET ?"#;
        }
        "no-reply" => {
            sql = r#"
                SELECT * FROM (
                    SELECT
                    t.id AS topic_id, user_id AS author_id, username AS author_name, avatar_url AS author_avatar_url, c.name AS category_name, title, view_count, t.create_time AS topic_create_time,
                    (SELECT count(comment.id) FROM comment WHERE comment.topic_id = t.id) AS comment_count,
                    t.sticky AS topic_sticky,
                    t.essence AS topic_essence
                    FROM topic AS t
                    LEFT JOIN category AS c
                    ON t.category_id = c.id
                    LEFT JOIN user AS u
                    ON t.user_id = u.id
                ) AS temp_table
                WHERE comment_count = 0
                ORDER BY topic_sticky DESC, topic_create_time DESC
                LIMIT ? OFFSET ?
                "#.to_string();
        }
        "ask" => {
            sql = sql_tpl_1 + " WHERE t.category_id = 1 " + sql_tpl_2;
        }
        "share" => {
            sql = sql_tpl_1 + " WHERE t.category_id = 2 " + sql_tpl_2;
        }
        "job" => {
            sql = sql_tpl_1 + " WHERE t.category_id = 3 " + sql_tpl_2;
        }
        _ => {
            sql = sql_tpl_1 + sql_tpl_2;
        }
    }

    let result = SQL_POOL.prep_exec(sql, (RECORDS_COUNT_PER_PAGE, offset)).unwrap();

    map_to_topic_list(result)
}

pub fn get_topic_list_count(tab_code: &str) -> u32 {
    let sql;

    match tab_code {
        "essence" => {
            sql = "SELECT count(id) FROM topic WHERE essence = 1";
        }
        "latest" => {
            sql = "SELECT count(id) FROM topic";
        }
        "no-reply" => {
            sql = r#"
                SELECT count(*) FROM (
                    SELECT
                    t.id AS topic_id, (SELECT count(comment.id) FROM comment WHERE comment.topic_id = t.id) AS comment_count
                    FROM topic AS t
                ) AS temp_table
                WHERE comment_count = 0
                "#;
        }
        "ask" => {
            sql = "SELECT count(id) FROM topic WHERE category_id = 1";
        }
        "share" => {
            sql = "SELECT count(id) FROM topic WHERE category_id = 2";
        }
        "job" => {
            sql = "SELECT count(id) FROM topic WHERE category_id = 3";
        }
        _ => {
            sql = "SELECT count(id) FROM topic";
        }
    }

    let mut result = SQL_POOL.prep_exec(sql, ()).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u32, )>(row);

    count
}

pub fn get_user_topic_list(tab_code: &str, user_id: u16, page: u32) -> Vec<Value> {

    let offset;
    let sql;

    if page <= 1 {
        offset = 0;
    } else {
        offset = (page - 1) * RECORDS_COUNT_PER_PAGE;
    }

    match tab_code {
        "comments" => {
            sql = r#"
                SELECT
                t.id AS topic_id, t.user_id AS author_id, username AS author_name, avatar_url AS author_avatar_url, c.name AS category_name, title, view_count, t.create_time,
                (SELECT count(comment.id) FROM comment WHERE comment.topic_id = t.id) AS comment_count,
                t.sticky AS topic_sticky,
                t.essence AS topic_essence,
                comment.create_time
                FROM topic AS t
                LEFT JOIN category AS c
                ON t.category_id = c.id
                LEFT JOIN user AS u
                ON t.user_id = u.id
                LEFT JOIN comment
                ON comment.topic_id = t.id
                WHERE t.id IN (
                    SELECT DISTINCT topic_id FROM comment WHERE user_id = ?
                )
                GROUP BY t.id
                ORDER BY comment.create_time DESC
                LIMIT ? OFFSET ?
                "#;
        }
        "collections" => {
            sql = r#"
                SELECT
                t.id AS topic_id, t.user_id AS author_id, username AS author_name, avatar_url AS author_avatar_url, c.name AS category_name, title, view_count, t.create_time,
                (SELECT count(comment.id) FROM comment WHERE comment.topic_id = t.id) AS comment_count,
                t.sticky AS topic_sticky,
                t.essence AS topic_essence
                FROM collection
                LEFT JOIN topic AS t
                ON collection.topic_id = t.id
                LEFT JOIN category AS c
                ON t.category_id = c.id
                LEFT JOIN user AS u
                ON t.user_id = u.id
                WHERE collection.user_id = ?
                ORDER BY collection.create_time DESC
                LIMIT ? OFFSET ?
                "#;
        }
        _ => {
            sql = r#"
                SELECT
                t.id AS topic_id, user_id AS author_id, username AS author_name, avatar_url AS author_avatar_url, c.name AS category_name, title, view_count, t.create_time,
                (SELECT count(comment.id) FROM comment WHERE comment.topic_id = t.id) AS comment_count,
                t.sticky AS topic_sticky,
                t.essence AS topic_essence
                FROM topic AS t
                LEFT JOIN category AS c
                ON t.category_id = c.id
                LEFT JOIN user AS u
                ON t.user_id = u.id
                WHERE t.user_id = ?
                ORDER BY t.create_time DESC
                LIMIT ? OFFSET ?
                "#;
        }
    }

    let result = SQL_POOL.prep_exec(sql, (user_id, RECORDS_COUNT_PER_PAGE, offset)).unwrap();

    map_to_topic_list(result)
}

pub fn get_user_topic_list_count(tab_code: &str, user_id: u16) -> u32 {
    let sql;

    match tab_code {
        "comments" => {
            sql = r#"
                SELECT count(*) FROM (
                    SELECT DISTINCT topic_id FROM comment WHERE user_id = ?
                ) AS temp_table
                "#;
        }
        "collections" => {
            sql = "SELECT count(id) FROM collection WHERE user_id = ?";
        }
        _ => {
            sql = "SELECT count(id) FROM topic WHERE user_id = ?";
        }
    }

    let mut result = SQL_POOL.prep_exec(sql, (user_id, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u32, )>(row);

    count
}

pub fn get_search_topic_list(keyword: &str, page: u32) -> Vec<Value> {

    let offset;

    if page <= 1 {
        offset = 0;
    } else {
        offset = (page - 1) * RECORDS_COUNT_PER_PAGE;
    }

    let sql = r#"
            SELECT
            t.id AS topic_id, user_id AS author_id, username AS author_name, avatar_url AS author_avatar_url, c.name AS category_name, title, view_count, t.create_time,
            (SELECT count(comment.id) FROM comment WHERE comment.topic_id = t.id) AS comment_count,
            t.sticky AS topic_sticky,
            t.essence AS topic_essence
            FROM topic AS t
            LEFT JOIN category AS c
            ON t.category_id = c.id
            LEFT JOIN user AS u
            ON t.user_id = u.id
            WHERE t.title REGEXP ? OR t.content REGEXP ?
            ORDER BY t.create_time DESC
            LIMIT ? OFFSET ?
            "#;

    let result = SQL_POOL.prep_exec(sql, (keyword, keyword, RECORDS_COUNT_PER_PAGE, offset)).unwrap();

    map_to_topic_list(result)
}

pub fn get_search_topic_list_count(keyword: &str) -> u32 {

    let sql = "SELECT count(id) FROM topic WHERE title REGEXP ? OR content REGEXP ?";

    let mut result = SQL_POOL.prep_exec(sql, (keyword, keyword)).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u32, )>(row);

    count
}

pub fn get_rss_topic_list() -> Vec<Item> {

    let sql = r#"
            SELECT
            t.id, username, title, content, t.create_time
            FROM topic AS t
            LEFT JOIN user AS u
            ON t.user_id = u.id
            ORDER BY t.create_time DESC
            LIMIT ? OFFSET ?
            "#;

    let result = SQL_POOL.prep_exec(sql, (RECORDS_COUNT_PER_PAGE, 0)).unwrap();

    let now = Local::now();
    let time_offset = now.offset().clone();

    result.map(|row_wrapper| row_wrapper.unwrap())
        .map(|mut row| {

            let topic_id = row.get::<String, _>(0).unwrap();
            let create_time = row.get::<NaiveDateTime, _>(4).unwrap();
            let topic_url = PATH.to_string() + "/topic/" + &*topic_id;
            let create_time_tz = DateTime::<Local>::from_utc(create_time - time_offset.fix(), time_offset);

            Item {
                author: Some(row.get::<String, _>(1).unwrap()),
                title: Some(row.get::<String, _>(2).unwrap()),
                description: Some(parse_to_html(&*row.get::<String, _>(3).unwrap())),
                link: Some(topic_url.clone()),
                guid: Some({Guid{
                    is_permalink: true,
                    value: topic_url
                }}),
                pub_date: Some(create_time_tz.to_rfc2822()),
                ..Default::default()
            }
        })
        .collect()
}

fn map_to_topic_list(result: QueryResult) -> Vec<Value> {

    result.map(|row_wrapper| row_wrapper.unwrap())
        .map(|mut row| {
            json!({
                "topic_id": row.get::<String, _>(0).unwrap(),
                "author_id": row.get::<u64, _>(1).unwrap(),
                "author_name": row.get::<String, _>(2).unwrap(),
                "author_avatar_url": row.get::<String, _>(3).unwrap(),
                "category_name": row.get::<String, _>(4).unwrap(),
                "title": row.get::<String, _>(5).unwrap(),
                "view_count": row.get::<u64, _>(6).unwrap(),
                "create_time": row.get::<NaiveDateTime, _>(7).unwrap(),
                "comment_count": row.get::<u64, _>(8).unwrap(),
                "sticky": row.get::<u64, _>(9).unwrap(),
                "essence": row.get::<u64, _>(10).unwrap()
            })
        })
        .collect()
}
