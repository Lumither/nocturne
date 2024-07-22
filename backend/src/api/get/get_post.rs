use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
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
                StatusCode::BAD_REQUEST,
                Json::from(json!(("error", e.to_string()))),
            ));
        }
    };

    let post = match query(
        r#"
SELECT jsonb_strip_nulls(to_jsonb(t.*)) - 'post_id' AS post
FROM post t
WHERE post_id = $1;
"#,
    )
    .bind(post_id)
    .fetch_one(&db_connection)
    .await
    {
        Ok(value) => value.get::<Value, _>("post"),
        Err(e) => {
            error!(
                "Database error on Reading `base post information` from table `post`: <{}>: {}",
                post_id.to_string(),
                e.to_string()
            );
            return match e {
                Error::RowNotFound => Err((
                    StatusCode::BAD_REQUEST,
                    Json::from(json!({"error": e.to_string()})),
                )),
                _ => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json::from(json!({"error": e.to_string()})),
                )),
            };
        }
    };

    let tags: Vec<_> = match query("SELECT tag FROM Tag WHERE post_id = $1;")
        .bind(post_id)
        .fetch_all(&db_connection)
        .await
    {
        Ok(tags) => tags.iter().map(|tag| tag.get::<String, _>("tag")).collect(),
        Err(e) => {
            error!("Database query error :{}", e.to_string());
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::from(json!({"error": e.to_string()})),
            ));
        }
    };

    let meta = match query(
        r#"
SELECT jsonb_strip_nulls(to_jsonb(t.*)) - 'post_id' AS meta
FROM meta t
WHERE post_id = $1;
        "#,
    )
    .bind(post_id)
    .fetch_one(&db_connection)
    .await
    {
        Ok(value) => value.get::<Value, _>("meta"),
        Err(e) => {
            error!(
                "Database error on Reading `{}` from table `{}`: <{}>: {}",
                "post meta data",
                "meta",
                post_id.to_string(),
                e.to_string()
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::from(json!({"error": e.to_string()})),
            ));
        }
    };

    // todo: next, prev post

    let mut res = post.as_object().unwrap().to_owned();
    res.insert("tags".to_string(), tags.into());
    res.insert("meta".to_string(), meta.into());

    Ok(Json::from(json!(res)))
}
