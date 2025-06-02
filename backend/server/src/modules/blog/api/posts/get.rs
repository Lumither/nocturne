use crate::{
    constants::blog::PAGE_SIZE,
    err_resp_log,
    utils::axum_response::{err_resp, succ_resp},
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, PgPool, query_as};
use uuid::Uuid;

#[derive(FromRow, Serialize)]
struct PostBriefModel {
    id: Uuid,
    identifier: String,
    title: String,
    subtitle: String,
    tags: Vec<String>,
    category: String,
    date_created: NaiveDate,
    date_updated: Option<NaiveDate>,
    header_img: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Pagination {
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Serialize)]
pub struct RespPagination {
    page: u32,
    page_size: u32,
    page_count: u32,
}

#[derive(Serialize)]
struct Response {
    posts: Vec<PostBriefModel>,
    pagination: RespPagination,
}

#[derive(FromRow)]
struct UuidRow {
    id: Uuid,
}

const GET_POST_WITH_BRIEF_MODEL: &str = include_str!("get_post_with_brief_model.sql");

pub async fn handler(
    State(db): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(PAGE_SIZE);
    let page_count = match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM posts")
        .fetch_one(&db)
        .await
    {
        Ok(post_count) => ((post_count as f64) / (page_size as f64)).ceil() as u32,
        Err(e) => return err_resp_log!(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    };

    let pagination = RespPagination {
        page,
        page_size,
        page_count,
    };

    if page > page_count {
        return err_resp(
            StatusCode::BAD_REQUEST,
            "page number should not be greater than page count",
            Some(json!({"pagination": pagination})),
        );
    }

    let offset = page_size * (page - 1);

    let ids = match query_as::<_, UuidRow>(
        "SELECT id FROM posts ORDER BY date_created DESC LIMIT $1 OFFSET $2",
    )
    .bind(page_size as i32)
    .bind(offset as i32)
    .fetch_all(&db)
    .await
    {
        Ok(res) => res.into_iter().map(|item| item.id).collect::<Vec<_>>(),
        Err(e) => return err_resp_log!(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    };

    let posts = match query_as::<_, PostBriefModel>(GET_POST_WITH_BRIEF_MODEL)
        .bind(&ids)
        .fetch_all(&db)
        .await
    {
        Ok(posts) => posts,
        Err(e) => return err_resp_log!(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    };

    let resp = Response { posts, pagination };
    succ_resp(StatusCode::OK, serde_json::to_value(resp).unwrap())
}
