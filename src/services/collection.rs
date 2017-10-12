use mysql::from_row;
use mysql::error::Error::MySqlError;

use common::utils::*;
use common::lazy_static::SQL_POOL;

pub fn create_collection(user_id: &str, topic_id: &str) -> Option<u8> {

    let create_time = gen_datetime().to_string();

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO collection
                        (user_id, topic_id, create_time)
                        VALUES
                        (?, ?, ?)
                        "#).unwrap();
    let result = stmt.execute((
        user_id,
        topic_id,
        &*create_time,
    ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(1)
}

pub fn delete_collection(user_id: &str, topic_id: &str) -> Option<u8> {

    let mut stmt = SQL_POOL.prepare(r#"
                        DELETE FROM collection
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

pub fn is_collected(user_id: &str, topic_id: &str) -> bool {

    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM collection WHERE user_id = ? AND topic_id = ?", (user_id, topic_id)).unwrap();
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
