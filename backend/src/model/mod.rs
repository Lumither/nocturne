use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod post;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub post_id: Uuid,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub last_update: NaiveDateTime,
    pub first_update: NaiveDateTime,
}


