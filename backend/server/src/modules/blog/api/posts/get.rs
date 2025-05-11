use macros::dev_consume;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn handler(
    State(db_connection): State<PgPool>,
    Path(identifier): Path<String>,
) -> impl IntoResponse {
    if let Ok(id) = identifier.parse::<Uuid>() {
        select_post_by_uuid(&db_connection, id).await
    } else {
        select_post_by_str_id(&db_connection, &identifier).await
    };
    Json::from(json!({"res": "ok"})).into_response()
}

async fn select_post_by_uuid(db: &PgPool, id: Uuid) {
    dev_consume!(db, id);
    todo!()
}

async fn select_post_by_str_id(db: &PgPool, id: &str) {
    dev_consume!(db, id);
    todo!()
}
