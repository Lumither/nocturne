use axum::extract::State;
use axum::Json;
use chrono::{DateTime, Utc};
use serde_json::{json, Map, Value};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn get_post_list(
    State(db_connection): State<PgPool>,
) -> Result<Json<Value>, Json<Value>> {
    let post_list =
        sqlx::query("select post_id, title, summary, last_update, first_update from post")
            .fetch_all(&db_connection)
            .await
            .unwrap();

    let mut res = Map::new();
    let posts: Vec<Value> = post_list
        .into_iter()
        .map(|post| {
            let mut tmp = Map::new();
            let post_id: Uuid = post.get("post_id");
            let title: String = post.get("title");
            let summary: String = post.get("summary");
            let last_update: DateTime<Utc> = post.get("last_update");
            let first_update: DateTime<Utc> = post.get("first_update");

            tmp.insert("post_id".to_string(), json!(post_id.to_string()));
            tmp.insert("title".to_string(), json!(title));
            tmp.insert("summary".to_string(), json!(summary));
            tmp.insert("last_update".to_string(), json!(last_update));
            tmp.insert("first_update".to_string(), json!(first_update));

            Value::from(tmp)
        })
        .collect();

    res.insert("posts".to_string(), Value::from(posts));
    Ok(Json::from(Value::from(res)))
}
