use mysql::from_row;
use mysql::error::Error::MySqlError;

use common::utils::*;
use common::lazy_static::SQL_POOL;

pub fn is_voted(user_id: &str, comment_id: &str) -> bool {

    let mut result = SQL_POOL.prep_exec(r#"
                        SELECT count(id) FROM comment_vote
                        WHERE
                        user_id = ? AND comment_id = ?
                        "#, (user_id, comment_id)).unwrap();

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

pub fn is_agreed(user_id: &str, comment_id: &str) -> bool {

    let mut result = SQL_POOL.prep_exec(r#"
                        SELECT count(id) FROM comment_vote
                        WHERE
                        user_id = ? AND comment_id = ? AND state = 1
                        "#, (user_id, comment_id)).unwrap();

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

pub fn is_disagreed(user_id: &str, comment_id: &str) -> bool {

    let mut result = SQL_POOL.prep_exec(r#"
                        SELECT count(id) FROM comment_vote
                        WHERE
                        user_id = ? AND comment_id = ? AND state = -1
                        "#, (user_id, comment_id)).unwrap();

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

pub fn create_comment_vote(user_id: &str, comment_id: &str, state: &str) -> Option<u8> {

    let create_time = gen_datetime().to_string();
    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO comment_vote
                        (user_id, comment_id, state, create_time, update_time)
                        VALUES
                        (?, ?, ?, ?, ?)
                        "#).unwrap();

    let result = stmt.execute((user_id, comment_id, state, &*create_time, &*create_time));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}


pub fn update_comment_vote(user_id: &str, comment_id: &str, state: &str) -> Option<u8> {

    let update_time = gen_datetime().to_string();
    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE comment_vote SET
                        state = ?,
                        update_time = ?
                        WHERE
                        user_id = ? AND comment_id = ?
                        "#).unwrap();

    let result = stmt.execute((state, &*update_time, user_id, comment_id));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}

pub fn delete_comment_vote(user_id: &str, comment_id: &str) -> Option<u8> {

    let mut stmt = SQL_POOL.prepare(r#"
                        DELETE FROM comment_vote
                        WHERE
                        user_id = ? AND comment_id = ?
                        "#).unwrap();

    let result = stmt.execute((user_id, comment_id));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}