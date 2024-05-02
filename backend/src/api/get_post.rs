use axum::extract::{Path, State};
use axum::Json;
use serde_json::Value;
use sqlx::PgPool;

pub async fn get_post(
    State(_db_connection): State<PgPool>,
    Path(post_id): Path<String>,
) -> Result<Json<Value>, Json<Value>> {
    dbg!(&post_id);
    todo!()
}
