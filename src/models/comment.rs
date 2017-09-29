use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: String,
    pub user_id: u16,
    pub topic_id: String,
    pub content: String,
    pub status: u8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}
