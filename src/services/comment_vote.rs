use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;

pub fn create_comment_vote(data: &Value) -> Option<u64> {

    Some(1)
}

pub fn update_comment_vote(data: &Value) -> Option<u64> {

    Some(1)
}