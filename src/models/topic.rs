use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Topic {
    pub id: String,
    pub user_id: u16,
    pub category_id: u8,
    pub title: String,
    pub content: String,
    pub status: u8,
    pub priority: u8,
    pub view_count: u32,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}
