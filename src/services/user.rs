use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;
use models::user::User;

pub fn check_user_login(username: &str, password: &str) -> Option<String> {
    let mut result = SQL_POOL.prep_exec(r#"
                            SELECT
                            password, salt
                            FROM
                            user
                            where username = ?
                            "#, (username, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (password_hashed, salt) = from_row::<(String, String)>(row);
    let password_with_salt = password.to_string() + &*salt;

    if password_hashed != gen_md5(&password_with_salt) {
        return None;
    }

    Some(username.to_string())
}

pub fn update_password(username: &str, password: &str) -> Option<String> {

    let salt = gen_salt();
    let password_with_salt = password.to_string() + &*salt;
    let password_hashed = gen_md5(&password_with_salt);
    let update_time = gen_datetime().to_string();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE user SET
                        password = ?,
                        salt = ?,
                        update_time = ?
                        WHERE username = ?
                        "#).unwrap();
    let result = stmt.execute((
        &*password_hashed,
        &*salt,
        &*update_time,
        username
    ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err.message);
        return None;
    }

    Some(username.to_string())
}

pub fn update_user(username: &str, user: &Value) -> Option<String> {

    let update_time = gen_datetime().to_string();
    let new_username = user["username"].as_str().unwrap();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE user SET
                        username = ?,
                        github_account = ?,
                        qq = ?,
                        email = ?,
                        site = ?,
                        avatar_url = ?,
                        location = ?,
                        signature = ?,
                        update_time = ?
                        WHERE username = ?
                        "#).unwrap();
    let result = stmt.execute((
        new_username,
        user["github_account"].as_str().unwrap(),
        user["qq"].as_str().unwrap(),
        user["email"].as_str().unwrap(),
        user["site"].as_str().unwrap(),
        user["avatar_url"].as_str().unwrap(),
        user["location"].as_str().unwrap(),
        user["signature"].as_str().unwrap(),
        &*update_time,
        username
    ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err.message);
        return None;
    }

    Some(new_username.to_string())
}

pub fn is_user_created(username: &str) -> bool {
    let mut result = SQL_POOL.prep_exec("SELECT count(id) from user where username = ?", (username, )).unwrap();
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

pub fn get_user_id(username: &str) -> u16 {
    let mut result = SQL_POOL.prep_exec("SELECT id from user where username = ?", (username, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (id, ) = from_row::<(u16, )>(row);

    id
}

pub fn get_user_id_by_github_id(id: u64) -> u16 {
    let mut result = SQL_POOL.prep_exec("SELECT user_id from github_user where id = ?", (id, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (user_id, ) = from_row::<(u16, )>(row);

    user_id
}

pub fn get_username(id: u16) -> Option<String> {
    let mut result = SQL_POOL.prep_exec("SELECT username from user where id = ?", (id, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (username, ) = from_row::<(String, )>(row);

    Some(username.to_string())
}

pub fn get_user(username: &str) -> Option<User> {
    let mut result = SQL_POOL.prep_exec(r#"
                          SELECT * from user where username = ?
                          "#, (username, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let mut row = row_wrapper.unwrap().unwrap();

    Some(User {
        id: row.get::<u16, _>(0).unwrap(),
        username: row.get::<String, _>(1).unwrap(),
        nickname: row.get::<String, _>(2).unwrap(),
        user_role: row.get::<u8, _>(3).unwrap(),
        register_source: row.get::<u8, _>(4).unwrap(),
        gender: row.get::<u8, _>(5).unwrap(),
        signature: row.get::<String, _>(6).unwrap(),
        email: row.get::<String, _>(7).unwrap(),
        avatar_url: row.get::<String, _>(8).unwrap(),
        qq: row.get::<String, _>(9).unwrap(),
        location: row.get::<String, _>(10).unwrap(),
        site: row.get::<String, _>(11).unwrap(),
        github_account: row.get::<String, _>(12).unwrap(),
        create_time: row.get::<NaiveDateTime, _>(15).unwrap(),
        update_time: row.get::<NaiveDateTime, _>(16).unwrap()
    })
}

pub fn create_user(user: &Value) -> Option<String> {

    let username = user["username"].as_str().unwrap();

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO user
                        (username, register_source, email, avatar_url, github_account, password, salt, create_time, update_time)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                        "#).unwrap();

    let result = stmt.execute((
        username,
        user["register_source"].as_u64().unwrap(),
        user["email"].as_str().unwrap(),
        user["avatar_url"].as_str().unwrap(),
        user["github_account"].as_str().unwrap(),
        user["password_hashed"].as_str().unwrap(),
        user["salt"].as_str().unwrap(),
        user["create_time"].as_str().unwrap(),
        user["create_time"].as_str().unwrap()
    ));

    if let Err(MySqlError(ref err)) = result {
        if err.code == 1062 {
            return None;
        } else {
            panic!("{:?}", err.message);
        }
    }

    Some(username.to_string())
}

pub fn is_github_user_binded(id: u64) -> bool {
    let mut result = SQL_POOL.prep_exec("SELECT count(id) from github_user where id = ?", (id, )).unwrap();
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

pub fn bind_github_user(user: &Value) -> Option<String> {
    let id = user["id"].as_u64().unwrap();
    let username = user["login"].as_str().unwrap();
    let nickname = user["name"].as_str().unwrap();
    let email = user["email"].as_str().unwrap();
    let avatar_url = user["avatar_url"].as_str().unwrap();
    let home_url = user["html_url"].as_str().unwrap();
    let create_time = &*gen_datetime().to_string();

    if is_user_created(username) {  // 该github用户名已被本站用户注册

        return None;
    }

    create_user(&json!({
        "username": user["login"],
        "register_source": 1,
        "email": user["email"],
        "avatar_url": user["avatar_url"],
        "github_account": user["login"],
        "password_hashed": "-".to_string(),
        "salt": "-".to_string(),
        "create_time": create_time
    }));

    let user_id = get_user_id(username);

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO github_user
                        (id, user_id, username, nickname, email, avatar_url, home_url, create_time, update_time)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                        "#).unwrap();
    let result = stmt.execute((
        id,
        user_id,
        username,
        nickname,
        email,
        avatar_url,
        home_url,
        create_time,
        create_time
    ));

    if let Err(MySqlError(ref err)) = result {
        if err.code == 1062 {
            return None;
        } else {
            panic!("{:?}", err.message);
        }
    }

    Some(username.to_string())
}

pub fn update_github_user(user: &Value) -> Option<String> {
    let id = user["id"].as_u64().unwrap();
    let username = user["login"].as_str().unwrap();
    let nickname = user["name"].as_str().unwrap();
    let email = user["email"].as_str().unwrap();
    let avatar_url = user["avatar_url"].as_str().unwrap();
    let home_url = user["html_url"].as_str().unwrap();
    let update_time = &*gen_datetime().to_string();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE github_user SET
                        username = ?,
                        nickname = ?,
                        email = ?,
                        avatar_url = ?,
                        home_url = ?,
                        update_time = ?
                        WHERE id = ?
                        "#).unwrap();
    let result = stmt.execute((
        username,
        nickname,
        email,
        avatar_url,
        home_url,
        update_time,
        id
    ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err.message);
        return None;
    }

    let user_id = get_user_id_by_github_id(id);
    get_username(user_id)
}