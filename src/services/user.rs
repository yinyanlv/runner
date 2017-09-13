use mysql::{Pool, from_row};
use mysql::error::Error::MySqlError;

use common::utils::*;

pub fn check_user_login(pool: &Pool, username: &str, password: &str) -> Option<u32> {

    let mut result = pool.prep_exec("SELECT id, password, salt FROM user where username = ?", (username,)).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (user_id, password_hashed, salt) = from_row::<(u32, String, String)>(row);
    let password_with_salt = password.to_string() + &*salt;

    if password_hashed != gen_md5(&password_with_salt) {
        return None;
    }

    Some(user_id)
}

pub fn create_user(pool: &Pool, fields: (String, String, String, String, String)) -> Option<u32> {

    let mut stmt = pool.prepare("INSERT INTO user (username, email, password, salt, create_time) VALUES (?, ?, ?, ?, ?)").unwrap();
    let result = stmt.execute(fields);

    if let Err(MySqlError(ref err)) = result {

        if err.code == 1062 {

            return None;
        }
    }

    Some(1)
}