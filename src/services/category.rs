use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;
use models::category::Category;

pub fn get_categories() -> Vec<Category> {

    let mut result = SQL_POOL.prep_exec("SELECT id, name from category", ()).unwrap();

    result.map(|mut row_wrapper| row_wrapper.unwrap())
        .map(|mut row| {

            Category {
                id: row.get::<u8, _>(0).unwrap(),
                name: row.get::<String, _>(1).unwrap()
            }
        })
        .collect()
}

