use chrono::{NaiveDateTime};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Topic {
    pub id: String,
    pub user_id: u16,
    pub category_id: u8,
    pub category_name: String,
    pub title: String,
    pub content: String,
    pub status: u8,
    pub essence: u8,
    pub view_count: u32,
    pub agree_count: u16,
    pub disagree_count: u16,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}
