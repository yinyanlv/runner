use chrono::*;

pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub create_time: NaiveDateTime
}