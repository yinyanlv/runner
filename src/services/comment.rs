use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;
use models::comment::Comment;

pub fn create_comment() {

}

pub fn update_comment() {

}

pub fn get_comment_count() -> u64 {

    let mut result = SQL_POOL.prep_exec("SELECT count(id) from comment", ()).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u64, )>(row);

    count
}