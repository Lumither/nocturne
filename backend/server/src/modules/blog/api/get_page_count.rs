use crate::constants::blog::PAGE_SIZE;

use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::{query, PgPool, Row};
use tracing::error;

pub async fn get_page_count(
    State(db_connection): State<PgPool>,
) -> Result<Json<Value>, Json<Value>> {
    match query("select cast(ceil(count(*) / $1) as integer) as res from post")
        .bind(PAGE_SIZE as f32)
        .fetch_one(&db_connection)
        .await
    {
        Ok(res) => {
            let res: i32 = res.get("res");
            Ok(Json::from(json!({"res": res})))
        }
        Err(e) => {
            // todo: rename
            error!("Failed to get blog page count: {}", e.to_string());
            Err(Json::from(json!({"error": e.to_string()})))
        }
    }
}
