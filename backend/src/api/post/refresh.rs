use std::{env, fs};
use std::path::Path;
use std::str::FromStr;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use futures::{stream, StreamExt};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, query};
use tracing::error;
use uuid::Uuid;

use crate::api::post::errors::refresh::PostIdxError;
use crate::api::post::utils::{build_post_idx, post};
use crate::model::blog::{POST_BASE_MODEL, POST_HASH_MODEL, POST_META_MODEL, TAG_MODEL};

pub async fn refresh(
    State(db_connection): State<PgPool>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    // search post
    let data_repo_dir = match env::var("GIT_WORK_DIR") {
        Ok(value) => value,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json::from(json!((
                    "error",
                    "error at parsing environment variable: GIT_WORK_DIR"
                ))),
            ));
        }
    };

    // generate post idx
    let md_path = search_md(Path::new(&data_repo_dir));

    let post_list_with_hash: Vec<(Map<_, _>, Vec<_>)> = md_path
        .into_iter()
        .filter_map(|post_path| match post::from_path(&post_path) {
            Ok(post) => match fs::read(&post_path) {
                Ok(bytes) => {
                    let mut hasher = Sha256::new();
                    hasher.update(bytes);
                    let hash = hasher.finalize().to_vec();
                    Some((post, hash))
                }
                Err(e) => {
                    error!(
                        "Failed to read file while hashing the post <{}>, skipping: {}",
                        post_path,
                        e.to_string()
                    );
                    None
                }
            },
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

    // db create
    query(POST_BASE_MODEL)
        .execute(&db_connection)
        .await
        .unwrap();
    query(POST_META_MODEL)
        .execute(&db_connection)
        .await
        .unwrap();
    query(TAG_MODEL).execute(&db_connection).await.unwrap();
    query(POST_HASH_MODEL)
        .execute(&db_connection)
        .await
        .unwrap();

    let stream = stream::iter(post_list_with_hash).for_each_concurrent(None, |(post, hash)| {
        let db_clone = db_connection.clone();
        async move {
            if let Err(e) = build_post_idx(&db_clone, post, hash).await {
                error!("{}", e.to_string());
            }
        }
    });

    stream.await;

    Ok(StatusCode::OK)
}

/// Search a dir, return a list of markdown files
///
/// # Arguments
/// * `entry`: dir path
///
/// # Returns
/// Vec<String, Global>
/// * `String`: path of a markdown file
///
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

async fn build_post_idx(
    db_connection: &PgPool,
    post: Map<String, Value>,
    hash: Vec<u8>,
) -> Result<(), PostIdxError> {
    let meta = &post["meta"];

    // extract post id
    let post_id = match Uuid::from_str(post["post_id"].as_str().unwrap()) {
        Ok(id) => id,
        Err(e) => {
            return Err(PostIdxError::InvalidUUID {
                id: post["post_id"].as_str().unwrap().to_string(),
                err_msg: e.to_string(),
            })
        }
    };

    if build_post_idx::hash(db_connection, post_id, hash).await? {
        build_post_idx::base(db_connection, &post, post_id, meta).await?;
        build_post_idx::tag(db_connection, post_id, meta).await?;
        build_post_idx::meta(db_connection, post_id, meta).await?;
    }

    Ok(())
}
