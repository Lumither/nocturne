use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::{query, Error, PgPool, Row};
use std::str::FromStr;
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

    let adjacent = match query(r"
WITH target_post AS (SELECT t.date FROM meta t WHERE post_id = $1),
     prev AS (SELECT post_id, t.title, t.sub_title, t.header_img
              FROM meta t
              WHERE t.date < (SELECT date FROM target_post)
              ORDER BY t.date DESC
              LIMIT 1),
     next AS (SELECT post_id, t.title, t.sub_title, t.header_img
              FROM meta t
              WHERE t.date > (SELECT date FROM target_post)
              ORDER BY t.date
              LIMIT 1)
SELECT json_strip_nulls(json_build_object(
        'prev',
        (SELECT json_build_object('id', post_id, 'title', title, 'sub_title', sub_title, 'header_img', header_img)
         FROM prev),
        'next',
        (SELECT json_build_object('id', post_id, 'title', title, 'sub_title', sub_title, 'header_img', header_img)
         FROM next)
                        )) AS result;
"
    )
        .bind(post_id)
        .fetch_one(&db_connection)
        .await
    {
        Ok(res) => {res.get::<Value, _>("result")}
        Err(e) => {
            error!(
                "Database error on Reading `{}` from table `{}`: <{}>: {}",
                "adjoint posts data",
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

    let mut res = post.as_object().unwrap().to_owned();
    res.insert("tags".to_string(), tags.into());
    res.insert("meta".to_string(), meta);
    res.insert("adj".to_string(), adjacent);

    Ok(Json::from(json!(res)))
}
