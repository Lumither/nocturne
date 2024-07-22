use axum::{extract::State, http::StatusCode, Json};
use axum::extract::Query;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sqlx::{PgPool, postgres::PgRow, query, Row};
use tracing::error;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Params {
    page: Option<u32>,
}

const PAGE_SIZE: u32 = 6;

pub async fn get_post_list(
    State(db_connection): State<PgPool>,
    Query(params): Query<Params>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let offset = PAGE_SIZE * (params.page.unwrap_or(1) - 1);

    let post_list = sqlx::query("SELECT * FROM post ORDER BY first_update DESC LIMIT $1 OFFSET $2")
        .bind(PAGE_SIZE as i32)
        .bind(offset as i32)
        .fetch_all(&db_connection)
        .await;
    let post_list = match post_list {
        Ok(post_list) => post_list,
        Err(e) => {
            error!(
                "Database query error on fetching post list: {}",
                e.to_string()
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::from(json!(("error", e.to_string()))),
            ));
        }
    };

    let mut res = Map::new();
    let posts: Vec<_> = post_list
        .into_iter()
        .map(|post| analyze_post(&db_connection, post))
        .collect();

    let posts = join_all(posts).await;

    res.insert("posts".to_string(), Value::from(posts));
    Ok(Json::from(Value::from(res)))
}

async fn analyze_post(db_connection: &PgPool, post: PgRow) -> Value {
    let post_id: Uuid = post.get("post_id");
    let tags = fetch_post_tags(db_connection, post_id).await;

    json!({
        "post_id": post_id.to_string(),
        "title": post.get::<String, _>("title"),
        "category": post.get::<String, _>("category"),
        "header_img": post.get::<String, _>("header_img"),
        "sub_title": post.get::<String, _>("sub_title"),
        "summary": post.get::<String, _>("summary"),
        "last_update": post.get::<DateTime<Utc>, _>("last_update"),
        "first_update": post.get::<DateTime<Utc>, _>("first_update"),
        "tags": tags,
    })
}

async fn fetch_post_tags(db_connection: &PgPool, post_id: Uuid) -> Vec<String> {
    match query("SELECT tag FROM Tag WHERE post_id = $1")
        .bind(post_id)
        .fetch_all(db_connection)
        .await
    {
        Ok(res) => res.iter().map(|tag| tag.get::<String, _>("tag")).collect(),
        Err(_) => vec![],
    }
}
