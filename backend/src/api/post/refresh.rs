use std::env;
use std::path::Path;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use sqlx::{PgPool, query};

use crate::model::Post;

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

    let post_list: Vec<Post> = md_list
        .into_iter()
        .map(|post| Post::from_path(post).unwrap())
        .collect();

    for post in post_list {
        match query(
            r##"
INSERT INTO Post (post_id, title, summary, content, last_update)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (post_id)
    DO UPDATE SET title       = excluded.title,
                  summary     = excluded.summary,
                  content     = excluded.content,
                  last_update = excluded.last_update
        "##,
        )
            .bind(post.post_id)
            .bind(post.title)
            .bind(post.summary)
            .bind(post.content)
            .bind(post.last_update)
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
