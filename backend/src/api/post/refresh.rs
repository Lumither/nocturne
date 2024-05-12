use std::env;
use std::path::Path;
use std::str::FromStr;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::DateTime;
use serde_json::{json, Map, Value};
use sqlx::{PgPool, query};
use uuid::Uuid;

use crate::constants::GLOBAL_TIME_FORMAT;
use crate::model::{post, POST_DB_MODEL, TAG_DB_MODEL};

pub async fn refresh(
    State(db_connection): State<PgPool>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    let data_repo_dir = match env::var("GIT_WORK_DIR") {
        Ok(value) => value,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::from(json!(format!(
                    "{{error: {}}}",
                    "error at parsing environment variable: GIT_WORK_DIR"
                ))),
            ));
        }
    };

    let md_list = search_md(Path::new(&data_repo_dir));

    // dump old idx
    query("DROP TABLE IF EXISTS Post;")
        .execute(&db_connection)
        .await
        .unwrap();
    query("DROP TABLE IF EXISTS Tag;")
        .execute(&db_connection)
        .await
        .unwrap();

    // generate new idx

    let post_list: Vec<Map<_, _>> = md_list
        .into_iter()
        .map(|post| post::from_path(post).unwrap())
        .collect();

    query(POST_DB_MODEL).execute(&db_connection).await.unwrap();

    query(TAG_DB_MODEL).execute(&db_connection).await.unwrap();

    // todo: concurrency
    for post in post_list {
        let meta = &post["meta"];

        // table Post
        match query(
            r##"
INSERT INTO Post (post_id, title, summary, content, last_update, first_update, sub_title)
VALUES ($1, $2, $3, $4, $5, $6, $7)
        "##,
        )
        .bind(Uuid::from_str(post["post_id"].as_str().unwrap()).unwrap())
        .bind(post["title"].as_str())
        .bind(
            post.get("summary")
                .and_then(|summary| summary.as_str())
                .unwrap_or(""),
        )
        .bind(post["content"].as_str())
        .bind(
            DateTime::parse_from_str(
                post["last_update"]
                    .as_str()
                    .expect("Failed to parse `last_update`"),
                GLOBAL_TIME_FORMAT,
            )
            .expect("Failed to convert `last_update` to NaiveDateTime"),
        )
        .bind(
            DateTime::parse_from_str(
                post["first_update"]
                    .as_str()
                    .expect("Failed to parse `first_update`"),
                GLOBAL_TIME_FORMAT,
            )
            .expect("Failed to convert `first_update` to NaiveDateTime"),
        )
        .bind(
            meta.get("sub_title")
                .and_then(|sub_title| sub_title.as_str())
                .unwrap_or(""),
        )
        .execute(&db_connection)
        .await
        {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json::from(json!(format!("{{error: {}}}", e.to_string()))),
                ));
            }
        }

        // table Tag
        if let Some(tags_string) = meta.get("tags").to_owned() {
            // parse tags string
            let parsed: Value = serde_json::from_str(tags_string.as_str().unwrap()).unwrap();
            let string_array = parsed.as_array().unwrap();
            let tags: Vec<String> = string_array
                .iter()
                .map(|value| value.as_str().unwrap().to_string())
                .collect();

            for tag in tags {
                match query(
                    r#"
INSERT INTO Tag
VALUES ($1, $2); 
                    "#,
                )
                .bind(Uuid::from_str(post["post_id"].as_str().unwrap()).unwrap())
                .bind(tag)
                .execute(&db_connection)
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json::from(json!(format!("{{error: {}}}", e.to_string()))),
                        ));
                    }
                }
            }
        }
    }

    Ok(StatusCode::OK)
}

fn search_md(entry: &Path) -> Vec<String> {
    if entry.ends_with(".git") {
        return vec![];
    }
    if entry.is_dir() {
        if let Ok(files) = entry.read_dir() {
            files
                .flat_map(|file| {
                    let file_path = file.unwrap().path();
                    search_md(file_path.as_path())
                })
                .collect()
        } else {
            vec![]
        }
    } else if entry.extension().is_some_and(|ext| ext == "md") {
        vec![entry.to_str().unwrap().to_string()]
    } else {
        vec![]
    }
}
