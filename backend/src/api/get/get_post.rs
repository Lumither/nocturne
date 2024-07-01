use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde_json::{json, Map, Value};
use sqlx::{Error, PgPool, query, Row};
use tracing::error;
use uuid::Uuid;

pub async fn get_post(
    State(db_connection): State<PgPool>,
    Path(post_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let post_id = match Uuid::from_str(post_id.as_str()) {
        Ok(id) => id,
        Err(e) => {
            error!(
                "Failed to parse UUID <{}>: {}",
                post_id.as_str(),
                e.to_string()
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::from(json!(("error", e.to_string()))),
            ));
        }
    };

    let meta = match query(
        r#"
SELECT *
FROM Post
WHERE post_id = $1;
    "#,
    )
    .bind(post_id)
    .fetch_one(&db_connection)
    .await
    {
        Ok(value) => value,
        Err(e) => {
            error!("Database query error on fetching meta: {}", e.to_string());
            return match e {
                Error::RowNotFound => Err((
                    StatusCode::BAD_REQUEST,
                    Json::from(json!(format!("{{error: {}}}", e.to_string()))),
                )),
                _ => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json::from(json!(format!("{{error: {}}}", e.to_string()))),
                )),
            };
        }
    };

    let tags = query(
        r#"
SELECT tag FROM Tag WHERE post_id = $1;
    "#,
    )
    .bind(post_id)
    .fetch_all(&db_connection)
    .await;
    let tags = match tags {
        Ok(tags) => tags,
        Err(e) => {
            error!("Database query error :{}", e.to_string());
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::from(json!(("error", e.to_string()))),
            ));
        }
    };

    let mut res = Map::new();

    res.insert(
        "title".to_string(),
        json!(meta.get::<String, &str>("title")),
    );
    res.insert(
        "sub_title".to_string(),
        json!(meta.get::<String, &str>("sub_title")),
    );
    res.insert(
        "category".to_string(),
        json!(meta.get::<String, &str>("category")),
    );
    res.insert(
        "header_img".to_string(),
        json!(meta.get::<String, &str>("header_img")),
    );
    res.insert(
        "summary".to_string(),
        json!(meta.get::<String, &str>("summary")),
    );
    res.insert(
        "content".to_string(),
        json!(meta.get::<String, &str>("content")),
    );
    res.insert(
        "last_update".to_string(),
        json!(meta.get::<DateTime<Utc>, &str>("last_update")),
    );
    res.insert(
        "first_update".to_string(),
        json!(meta.get::<DateTime<Utc>, &str>("first_update")),
    );

    let tags: Vec<String> = tags
        .iter()
        .map(|tag| {
            let tag: String = tag.get("tag");
            tag
        })
        .collect();
    res.insert("tags".to_string(), Value::from(tags));

    Ok(Json::from(Value::from(res)))
}
