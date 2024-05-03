use axum::extract::State;
use axum::Json;
use chrono::NaiveDateTime;
use serde_json::{json, Map, Value};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn get_post_list(
    State(db_connection): State<PgPool>,
) -> Result<Json<Value>, Json<Value>> {
    let post_list = sqlx::query("select post_id, title, summary, last_update from post")
        .fetch_all(&db_connection)
        .await
        .unwrap();

    let mut res = Map::new();
    for post in post_list {
        let mut tmp = Map::new();
        let post_id: Uuid = post.get("post_id");
        let title: String = post.get("title");
        let summary: String = post.get("summary");
        let last_update: NaiveDateTime = post.get("last_update");

        tmp.insert("title".to_string(), json!(title));
        tmp.insert("summary".to_string(), json!(summary));
        tmp.insert("last_update".to_string(), json!(last_update));

        res.insert(post_id.to_string(), Value::from(tmp));
    }

    Ok(Json::from(Value::from(res)))
}
