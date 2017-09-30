use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: String,
    pub user_id: u16,
    pub username: String,
    pub avatar_url: String,
    pub topic_id: String,
    pub content: String,
    pub agree_count: u16,
    pub disagree_count: u16,
    pub status: u8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}
