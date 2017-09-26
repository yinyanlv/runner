use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u16,
    pub username: String,
    pub nickname: String,
    pub user_role: u8,
    pub register_source: u8,
    pub gender: u8,
    pub signature: String,
    pub email: String,
    pub avatar_url: String,
    pub qq: String,
    pub location: String,
    pub site: String,
    pub github_account: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}
