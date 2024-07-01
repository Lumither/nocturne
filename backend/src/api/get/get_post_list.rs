use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use futures::future::join_all;
use serde_json::{json, Map, Value};
use sqlx::{postgres::PgRow, query, PgPool, Row};
use tracing::error;
use uuid::Uuid;

pub async fn get_post_list(
    State(db_connection): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let post_list = sqlx::query(
        "select post_id, title, sub_title, summary, last_update, first_update, header_img, category from post",
    )
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
    let mut tmp = Map::new();
    let post_id: Uuid = post.get("post_id");
    let title: String = post.get("title");
    let category: String = post.get("category");
    let header_img: String = post.get("header_img");
    let sub_title: String = post.get("sub_title");
    let summary: String = post.get("summary");
    let last_update: DateTime<Utc> = post.get("last_update");
    let first_update: DateTime<Utc> = post.get("first_update");

    let tags_future = query("SELECT tag FROM Tag WHERE post_id = $1")
        .bind(post_id)
        .fetch_all(db_connection);
    let tags: Vec<String> = match tags_future.await {
        Ok(res) => res.iter().map(|tag| tag.get::<String, _>("tag")).collect(),
        Err(_) => {
            vec![]
        }
    };

    tmp.insert("post_id".to_string(), json!(post_id.to_string()));
    tmp.insert("title".to_string(), json!(title));
    tmp.insert("category".to_string(), json!(category));
    tmp.insert("header_img".to_string(), json!(header_img));
    tmp.insert("sub_title".to_string(), json!(sub_title));
    tmp.insert("summary".to_string(), json!(summary));
    tmp.insert("last_update".to_string(), json!(last_update));
    tmp.insert("first_update".to_string(), json!(first_update));
    tmp.insert("tags".to_string(), json!(tags));

    Value::from(tmp)
}
