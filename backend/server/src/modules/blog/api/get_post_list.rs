use std::error::Error;

use crate::constants::blog::PAGE_SIZE;

use axum::{extract::Query, extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use futures::future::join_all;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sqlx::{postgres::PgRow, query, PgPool, Row};
use tracing::error;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Params {
    page: Option<u32>,
}

#[axum::debug_handler]
pub async fn get_post_list(
    State(db_connection): State<PgPool>,
    Query(params): Query<Params>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let search_param_page = params.page.unwrap_or(1);
    if search_param_page < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json::from(json!({"error": "`page` should be greater than 0"})),
        ));
    }

    let offset = PAGE_SIZE * (search_param_page - 1);

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
                Json::from(json!({"error": e.to_string()})),
            ));
        }
    };

    let mut res = Vec::new();
    let posts: Vec<_> = post_list
        .into_iter()
        .map(|post| async { analyze_post(&db_connection, post).await })
        .collect();
    let posts = join_all(posts).await;
    for post in posts {
        match post {
            Ok(p) => res.push(p),
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json::from(json!(("error", e.to_string()))),
                ))
            }
        }
    }

    // res.insert("posts".to_string(), Value::from(posts));
    Ok(Json::from(Value::from(res)))
}

async fn analyze_post(db_connection: &PgPool, post: PgRow) -> Result<Value, Box<dyn Error + Send>> {
    let post_id: Uuid = post.get("post_id");
    let tags = fetch_post_tags(db_connection, post_id).await;
    let meta = fetch_post_meta(db_connection, post_id).await.unwrap();
    let header_img = meta.get("header_img");
    let last_update = meta.get("last_update");

    Ok(json!({
        "post_id": post_id.to_string(),
        "title": post.get::<String, _>("title"),
        "category": post.get::<String, _>("category"),
        "header_img": header_img,
        "sub_title": post.get::<String, _>("sub_title"),
        "summary": post.get::<String, _>("summary"),
        "last_update": last_update,
        "first_update": post.get::<DateTime<Utc>, _>("first_update"),
        "tags": tags,
    }))
}

async fn fetch_post_meta(db_connection: &PgPool, post_id: Uuid) -> Option<Value> {
    match query(
        r#"
SELECT jsonb_strip_nulls(to_jsonb(t.*)) - 'post_id' AS meta
FROM meta t
WHERE post_id = $1;
        "#,
    )
    .bind(post_id)
    .fetch_one(db_connection)
    .await
    {
        Ok(value) => Some(value.get::<Value, _>("meta")),
        Err(e) => {
            error!(
                "Database error on Reading `{}` from table `{}`: <{}>: {}",
                "post meta data",
                "meta",
                post_id.to_string(),
                e.to_string()
            );
            None
        }
    }
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
