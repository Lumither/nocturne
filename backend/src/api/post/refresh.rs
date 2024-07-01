use std::env;
use std::path::Path;
use std::str::FromStr;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::DateTime;
use serde_json::{json, Map, Value};
use sqlx::{PgPool, query};
use tracing::error;
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
        .filter_map(|post_path| match post::from_path(&post_path) {
            Ok(post) => Some(post),
            Err(e) => {
                error!(
                    "Failed to parse post <{}>, skipping: {}",
                    post_path,
                    e.to_string()
                );
                None
            }
        })
        .collect();

    query(POST_DB_MODEL).execute(&db_connection).await.unwrap();

    query(TAG_DB_MODEL).execute(&db_connection).await.unwrap();

    // todo: concurrency
    for post in post_list {
        let meta = &post["meta"];

        let post_id = match Uuid::from_str(post["post_id"].as_str().unwrap()) {
            Ok(id) => id,
            Err(e) => {
                error!(
                    "Failed to parse UUID <{}>: {}",
                    post["post_id"].as_str().unwrap(),
                    e.to_string()
                );
                continue;
            }
        };
        let title = post["title"].as_str();
        let summary = post
            .get("summary")
            .and_then(|summary| summary.as_str())
            .unwrap_or("");
        let content = post["content"].as_str();
        let last_update = match DateTime::parse_from_str(
            post["last_update"].as_str().unwrap(),
            GLOBAL_TIME_FORMAT,
        ) {
            Ok(time) => time,
            Err(e) => {
                error!(
                    "Failed to parse last update time for <{}> on db table initialization: {}",
                    post_id.to_string(),
                    e.to_string()
                );
                continue;
            }
        };
        let first_update = match DateTime::parse_from_str(
            post["first_update"].as_str().unwrap(),
            GLOBAL_TIME_FORMAT,
        ) {
            Ok(time) => time,
            Err(e) => {
                error!(
                    "Failed to parse first update time for <{}> on db table initialization: {}",
                    post_id.to_string(),
                    e.to_string()
                );
                continue;
            }
        };
        let sub_title = meta
            .get("sub_title")
            .and_then(|sub_title| sub_title.as_str())
            .unwrap_or("");
        let category = meta
            .get("category")
            .and_then(|category| category.as_str())
            .unwrap_or("N/A");
        let header_img = meta
            .get("header_img")
            .and_then(|header_img| header_img.as_str())
            .unwrap_or("");

        // table Post
        match query(
            r##"
INSERT INTO Post (post_id, title, summary, content, last_update, first_update, sub_title, category, header_img)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "##,
        )
        .bind(post_id)
        .bind(title)
        .bind(summary)
        .bind(content)
        .bind(last_update)
        .bind(first_update)
        .bind(sub_title)
        .bind(category)
        .bind(header_img)
        .execute(&db_connection)
        .await
        {
            Ok(_) => {}
            Err(e) => {
                error!("Database error on initializing post data (insert):<{}> : {}", post_id.to_string(), e.to_string());
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json::from(json!(("error:", e.to_string()))),
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
