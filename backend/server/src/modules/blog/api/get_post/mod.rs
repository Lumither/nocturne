use macros::dev_consume;

use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use sqlx::PgPool;

const GET_POST_BY_I64_ID: &str = include_str!("get_post_by_i64_id.sql");
const GET_POST_BY_STR_ID: &str = include_str!("get_post_by_str_id.sql");

pub async fn handler(
    State(db_connection): State<PgPool>,
    Path(identifier): Path<String>,
) -> impl IntoResponse {
    if let Ok(id) = identifier.parse::<i64>() {
        select_post_with_i64_id(&db_connection, id).await
    } else {
        select_post_with_str_id(&db_connection, &identifier).await
    };
    Json::from(json!({"res": "ok"})).into_response()
}

async fn select_post_with_i64_id(db: &PgPool, id: i64) {
    dev_consume!(db, id);
    todo!()
}

async fn select_post_with_str_id(db: &PgPool, id: &str) {
    dev_consume!(db, id);
    todo!()
}
