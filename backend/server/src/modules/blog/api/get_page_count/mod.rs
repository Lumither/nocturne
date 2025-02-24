use crate::constants::blog::PAGE_SIZE;

use axum::{Json, extract::State};
use serde_json::{Value, json};
use sqlx::{PgPool, Row, query};
use tracing::error;

const GET_PAGE_COUNT_QUERY: &str = include_str!("post_page_count.sql");

pub async fn handler(State(db_connection): State<PgPool>) -> Result<Json<Value>, Json<Value>> {
    match query(GET_PAGE_COUNT_QUERY)
        .bind(PAGE_SIZE as i64)
        .fetch_one(&db_connection)
        .await
    {
        Ok(res) => {
            let res: i32 = res.get("count");
            Ok(Json::from(json!({"res": res})))
        }
        Err(e) => {
            error!("failed to get blog page count: {}", e.to_string());
            Err(Json::from(json!({"error": e.to_string()})))
        }
    }
}
