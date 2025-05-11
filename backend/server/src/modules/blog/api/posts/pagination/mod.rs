use crate::{
    constants::blog::PAGE_SIZE,
    utils::axum_response::{err_resp, succ_resp},
};

use axum::{extract::State, http::StatusCode, response::Response};
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::{FromRow, PgPool, Row, query_as};

#[derive(FromRow, Serialize)]
struct DBPaginationRow {
    page_count: i32,
    total_count: i64,
}

impl From<DBPaginationRow> for Value {
    fn from(value: DBPaginationRow) -> Self {
        json!({
            "page_count": value.page_count,
            "total_count": value.total_count
        })
    }
}

pub async fn handler(State(db_connection): State<PgPool>) -> Response<String> {
    match query_as::<_, DBPaginationRow>(
        r#"
        SELECT 
            cast(ceil(count(*) / cast($1 AS FLOAT)) AS INTEGER) AS page_count,
            count(*) AS total_count
        FROM posts;
    "#,
    )
    .bind(PAGE_SIZE as i64)
    .fetch_one(&db_connection)
    .await
    {
        Ok(res) => succ_resp(StatusCode::OK, res.into()),
        Err(e) => err_resp(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
