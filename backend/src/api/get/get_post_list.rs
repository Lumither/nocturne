use axum::extract::State;
use axum::Json;
use serde_json::Value;
use sqlx::PgPool;

use crate::model::Post;

pub async fn get_post_list(
    State(db_connection): State<PgPool>,
    Json(_request): Json<Value>,
) -> Result<Json<Value>, Json<Value>> {
    let post_list: Vec<Post> = sqlx::query_as("select * from post")
        .fetch_all(&db_connection)
        .await
        .unwrap();
    dbg!(&post_list);
    Ok(Json::from(serde_json::to_value(post_list).unwrap()))
}
