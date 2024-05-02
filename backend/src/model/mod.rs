use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    post_id: Uuid,
    title: String,
    content: String,
    last_update: NaiveDateTime,
}
