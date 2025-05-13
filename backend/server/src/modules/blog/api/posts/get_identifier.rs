use crate::{err_resp_log, utils::axum_response::succ_resp};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use futures::future::OptionFuture;
use serde_json::Value;
use sqlx::{FromRow, PgPool, query_as};
use uuid::Uuid;

const SELECT_POST_BY_UUID: &str = include_str!("get_post_by_uuid.sql");
const SELECT_POST_BY_STR_ID: &str = include_str!("get_post_by_str_id.sql");
const SELECT_POST_ADJACENT_IDS: &str = include_str!("get_post_adjacent_ids.sql");
const SELECT_POST_WITH_ADJACENT_MODEL: &str = include_str!("get_post_with_adjacent_model.sql");

#[derive(serde::Serialize, FromRow)]
struct SelectedPost {
    id: Uuid,
    identifier: String,
    title: String,
    subtitle: String,
    status: String,
    date_created: NaiveDate,
    date_updated: Option<NaiveDate>,
    category: String,
    tags: Vec<String>,
    content: String,
    metadata: Value,
}

#[derive(serde::Serialize, FromRow)]
struct AdjacentPost {
    id: Uuid,
    identifier: String,
    title: String,
    subtitle: String,
    date_created: NaiveDate,
    category: String,
    tags: Vec<String>,
    header_img: Option<String>,
}

#[derive(FromRow, Debug)]
struct AdjacentIds {
    prev: Option<Uuid>,
    next: Option<Uuid>,
}

#[derive(serde::Serialize, Default)]
struct AdjacentPosts {
    prev: Option<AdjacentPost>,
    next: Option<AdjacentPost>,
}

#[derive(serde::Serialize)]
struct ResponseModel {
    post: SelectedPost,
    adjacent: AdjacentPosts,
}

pub async fn handler(
    State(db_connection): State<PgPool>,
    Path(identifier): Path<String>,
) -> impl IntoResponse {
    let post = match if let Ok(id) = identifier.parse::<Uuid>() {
        select_post_by_uuid(&db_connection, id).await
    } else {
        select_post_by_str_id(&db_connection, &identifier).await
    } {
        Ok(post) => post,
        Err(e) => {
            return match e {
                sqlx::Error::RowNotFound => {
                    err_resp_log!(StatusCode::NOT_FOUND, "no matching post")
                }
                _ => err_resp_log!(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
            };
        }
    };

    let adj_ids = query_as::<_, AdjacentIds>(SELECT_POST_ADJACENT_IDS)
        .bind(post.id)
        .fetch_one(&db_connection)
        .await
        .unwrap();
    let adjacent_prev = OptionFuture::from(
        adj_ids
            .prev
            .map(|id| select_post_with_adjacent_model(&db_connection, id)),
    )
    .await
    .flatten();
    let adjacent_next = OptionFuture::from(
        adj_ids
            .next
            .map(|id| select_post_with_adjacent_model(&db_connection, id)),
    )
    .await
    .flatten();
    let adjacent = AdjacentPosts {
        prev: adjacent_prev,
        next: adjacent_next,
    };

    let response = ResponseModel { post, adjacent };
    succ_resp(StatusCode::OK, serde_json::to_value(response).unwrap())
}

async fn select_post_by_uuid(db: &PgPool, id: Uuid) -> Result<SelectedPost, sqlx::Error> {
    query_as::<_, SelectedPost>(SELECT_POST_BY_UUID)
        .bind(id)
        .fetch_one(db)
        .await
}

async fn select_post_by_str_id(db: &PgPool, id: &str) -> Result<SelectedPost, sqlx::Error> {
    query_as::<_, SelectedPost>(SELECT_POST_BY_STR_ID)
        .bind(id)
        .fetch_one(db)
        .await
}

async fn select_post_with_adjacent_model(db: &PgPool, id: Uuid) -> Option<AdjacentPost> {
    query_as(SELECT_POST_WITH_ADJACENT_MODEL)
        .bind(id)
        .fetch_optional(db)
        .await
        .unwrap()
}
