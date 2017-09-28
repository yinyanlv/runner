use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: u64,
    pub user_id: u16,
    pub topic_id: u32,
    pub content: String,
    pub status: u8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}
