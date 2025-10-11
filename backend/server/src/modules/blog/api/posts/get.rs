use crate::{
    constants::blog::PAGE_SIZE,
    err_resp_log,
    utils::axum_response::{err_resp, succ_resp},
};

use std::collections::HashSet;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::Query;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query_as, FromRow, PgPool};
use uuid::Uuid;

fn tags_deserializer<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deserialize_each = |input: String| -> Result<Vec<String>, D::Error> {
        Ok(input.split(',').map(|s| s.to_string()).collect())
    };

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Cardinality {
        One(String),
        Many(Vec<String>),
    }
    let cardinality: Option<Cardinality> = Deserialize::deserialize(deserializer)?;

    let mut tags = HashSet::new();

    match cardinality {
        Some(Cardinality::One(s)) => {
            for item in deserialize_each(s)? {
                let item = item.trim().to_string();
                if !item.is_empty() {
                    tags.insert(item);
                }
            }
            let ret = tags.into_iter().collect::<Vec<_>>();
            Ok(if ret.is_empty() { None } else { Some(ret) })
        }
        Some(Cardinality::Many(v)) => {
            for items in v
                .into_iter()
                .map(deserialize_each)
                .collect::<Result<Vec<_>, _>>()?
            {
                for item in items {
                    let item = item.trim().to_string();
                    if !item.is_empty() {
                        tags.insert(item);
                    }
                }
            }
            let ret = tags.into_iter().collect::<Vec<_>>();
            Ok(if ret.is_empty() { None } else { Some(ret) })
        }
        None => Ok(None),
    }
}

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
pub struct SearchParams {
    page: Option<u32>,
    page_size: Option<u32>,

    #[serde(default, deserialize_with = "tags_deserializer")]
    tags: Option<Vec<String>>,

    #[serde(default)]
    match_all: bool,
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

const I32_PLACEHOLDER: i32 = 0;

const COUNT_TAG_FILTERED_POSTS: &str = include_str!("count_tag_filtered_posts.sql");
const SELECT_POST_WITH_TAG_FILTER_PAGINATED: &str =
    include_str!("select_post_with_tag_filter_paginated.sql");
const GET_POST_WITH_BRIEF_MODEL: &str = include_str!("get_post_with_brief_model.sql");

pub async fn handler(
    State(db): State<PgPool>,
    Query(search_params): Query<SearchParams>,
) -> impl IntoResponse {
    let page = search_params.page.unwrap_or(1);
    let page_size = search_params.page_size.unwrap_or(PAGE_SIZE);

    let page_count = match sqlx::query_scalar::<_, i64>(COUNT_TAG_FILTERED_POSTS)
        .bind(&search_params.tags)
        .bind(I32_PLACEHOLDER) // not used, but help sqlx/psql hit cache
        .bind(I32_PLACEHOLDER)
        .bind(search_params.match_all)
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

    let ids = match query_as::<_, UuidRow>(SELECT_POST_WITH_TAG_FILTER_PAGINATED)
        .bind(search_params.tags)
        .bind(page_size as i32)
        .bind(offset as i32)
        .bind(search_params.match_all)
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
