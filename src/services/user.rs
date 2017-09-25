use mysql::{Pool, from_row};
use mysql::error::Error::MySqlError;
use serde_json::Value;

use common::utils::*;

pub fn check_user_login(pool: &Pool, username: &str, password: &str) -> Option<u32> {

    let mut result = pool.prep_exec("SELECT id, password, salt FROM user where username = ?", (username, )).unwrap();
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

pub fn create_user(pool: &Pool, user: &Value) -> Option<u32> {

    let mut stmt = pool.prepare("INSERT INTO user (username, email, password, salt, create_time) VALUES (?, ?, ?, ?, ?)").unwrap();
    let result = stmt.execute((
        user["username"].as_str().unwrap(),
        user["email"].as_str().unwrap(),
        user["password_hashed"].as_str().unwrap(),
        user["salt"].as_str().unwrap(),
        user["create_time"].as_str().unwrap()
    ));

    if let Err(MySqlError(ref err)) = result {

        if err.code == 1062 {

            return None;
        } else {

            panic!("{:?}", err.message);
        }
    }

    Some(1)
}

pub fn is_github_user_binded(pool: &Pool, id: u64) -> bool {

    let mut result = pool.prep_exec("SELECT count(*) from github_user where id = ?", (id, )).unwrap();
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

pub fn bind_github_user(pool: &Pool, user: &Value) -> Option<u8> {

    let id = user["id"].as_u64().unwrap();
    let username = user["login"].as_str().unwrap();
    let nickname = user["name"].as_str().unwrap();
    let email = user["email"].as_str().unwrap();
    let avatar_url = user["avatar_url"].as_str().unwrap();
    let home_url = user["html_url"].as_str().unwrap();
    let bind_time = &*gen_datetime().to_string();

    let mut stmt = pool.prepare("INSERT INTO github_user (id, username, nickname, email, avatar_url, home_url, bind_time) VALUES (?, ?, ?, ?, ?, ?, ?)").unwrap();
    let result = stmt.execute((
        id,
        username,
        nickname,
        email,
        avatar_url,
        home_url,
        bind_time
    ));

    if let Err(MySqlError(ref err)) = result {

        if err.code == 1062 {

            return None;
        } else {

            panic!("{:?}", err.message);
        }
    }

    Some(1)
}