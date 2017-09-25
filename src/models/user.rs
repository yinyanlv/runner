use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub avatar_url: String,
    pub create_time: String
}
