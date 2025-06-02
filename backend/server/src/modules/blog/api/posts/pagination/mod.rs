use crate::{constants::blog::PAGE_SIZE, err_resp_log, utils::axum_response::succ_resp};

use axum::{extract::State, http::StatusCode, response::Response};
use serde_json::{Value, json};
use sqlx::{FromRow, PgPool, query_as};

#[derive(FromRow)]
struct DBPaginationRow {
    page_count: i32,
    post_count: i64,
}

struct PaginationResponse {
    page_count: i32,
    post_count: i64,
}

impl From<DBPaginationRow> for PaginationResponse {
    fn from(value: DBPaginationRow) -> Self {
        Self {
            page_count: value.page_count,
            post_count: value.post_count,
        }
    }
}

impl From<PaginationResponse> for Value {
    fn from(value: PaginationResponse) -> Self {
        json!({
            "page_count": value.page_count,
            "post_count": value.post_count
        })
    }
}

pub async fn handler(State(db_connection): State<PgPool>) -> Response<String> {
    match query_as::<_, DBPaginationRow>(
        r#"
        SELECT
            cast(ceil(count(*) / cast($1 AS FLOAT)) AS INTEGER) AS page_count,
            count(*) AS post_count
        FROM posts;
    "#,
    )
    .bind(PAGE_SIZE as i64)
    .fetch_one(&db_connection)
    .await
    {
        Ok(res) => succ_resp(StatusCode::OK, PaginationResponse::from(res).into()),
        Err(e) => err_resp_log!(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}
