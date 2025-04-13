use crate::modules::blog::components::check_update::{
    changes::{Change, Create, Delete, Move, Update},
    error::Error,
    utils::{parse_naive_date_str, parse_post_identifier},
};
use macros::dev_consume;

use futures::{
    future::{join_all, BoxFuture},
    FutureExt,
};
use sqlx::{query, PgPool, Postgres, Row, Transaction};
use tracing::error;

const DELETE_POST_BY_ID: &str = include_str!("sql/delete_post_by_id.sql");
const GET_TAG_ID: &str = include_str!("sql/get_tag_id.sql");
const GET_CATEGORY_ID: &str = include_str!("sql/get_category_id.sql");
const CREATE_RETURN_TAG_ID: &str = include_str!("sql/create_return_tag_id.sql");
const CREATE_RETURN_CATEGORY_ID: &str = include_str!("sql/create_return_category_id.sql");
const CREATE_POST: &str = include_str!("sql/create_post.sql");

pub async fn apply_deltas(db_conn: &PgPool, changes: Vec<Change>) {
    let handlers = changes
        .into_iter()
        .map(|change| match change {
            Change::Delete(del) => handle_delete(db_conn, del).boxed(),
            Change::Move(mv) => handle_move(db_conn, mv).boxed(),
            Change::Update(update) => handle_update(db_conn, update).boxed(),
            Change::Create(create) => handle_create(db_conn, create).boxed(),
        })
        .collect::<Vec<BoxFuture<'_, ()>>>();

    join_all(handlers).await;
}

async fn handle_delete(db: &PgPool, del: Delete) {
    if let Err(e) = query(DELETE_POST_BY_ID).bind(del.uuid).execute(db).await {
        error!(
            "failed to delete post: {}, {}",
            del.uuid.to_string(),
            e.to_string()
        )
    }
}

async fn handle_move(db: &PgPool, mv: Move) {
    handle_delete(
        db,
        Delete {
            uuid: mv.uuid,
            path: mv.from,
        },
    )
    .await;
    handle_create(
        db,
        Create {
            uuid: mv.uuid,
            path: mv.to,
            payload: mv.payload,
        },
    )
    .await;
}

async fn handle_update(db: &PgPool, update: Update) {
    handle_delete(
        db,
        Delete {
            uuid: update.uuid,
            path: update.path.clone(),
        },
    )
    .await;
    handle_create(
        db,
        Create {
            uuid: update.uuid,
            path: update.path,
            payload: update.payload,
        },
    )
    .await;
}

async fn handle_create(db: &PgPool, create: Create) {
    let mut tx = db.begin().await.unwrap();

    let identifier = parse_post_identifier(&create.path).unwrap();

    let tags_json = create.payload.meta["tags"].clone();
    let tags: Vec<_> = if let Some(tags) = tags_json.as_array() {
        tags.iter().filter_map(|tag| tag.as_str()).collect()
    } else {
        vec![]
    };
    let mut tag_ids = vec![];
    for tag in tags {
        match get_or_create_tag_id(&mut tx, tag).await {
            Ok(tid) => tag_ids.push(tid),
            Err(e) => {
                error!("failed to fetch or create tag ({}): {}", tag, e);
                return;
            }
        }
    }

    let category_json = create.payload.meta["category"].clone();
    let category = category_json.as_str().unwrap_or("Uncategorized");
    let category_id = match get_or_create_category_id(&mut tx, category).await {
        Ok(id) => id,
        Err(e) => {
            error!(
                "failed to fetch category id for post {}: {}",
                &identifier, e
            );
            return;
        }
    };

    let meta = create.payload.meta;
    let post_id = create.uuid;
    let post_identifier = &identifier;
    let post_title = meta
        .get("title")
        .map(|t| t.as_str().unwrap())
        .unwrap_or("Untitled");
    let post_subtitle = meta
        .get("subtitle")
        .map(|t| t.as_str().unwrap())
        .unwrap_or("");
    let post_date_created = if let Some(Some(date)) = meta
        .get("date")
        .map(|d| parse_naive_date_str(d.as_str().unwrap_or("")))
    {
        date
    } else {
        error!("failed to parse `date` for post: {}", post_identifier);
        return;
    };
    let post_date_updated = meta
        .get("update")
        .map(|d| parse_naive_date_str(d.as_str().unwrap_or("")))
        .unwrap_or(None);
    let post_status = meta.get("status").map(|v| v.as_str()).unwrap_or(None);
    let post_content = create.payload.content;

    if let Err(e) = query(CREATE_POST)
        .bind(post_id)
        .bind(post_identifier)
        .bind(post_title)
        .bind(post_subtitle)
        .bind(post_date_created)
        .bind(post_date_updated)
        .bind(post_status)
        .bind(post_content)
        .bind(category_id)
        .bind(tag_ids)
        .execute(&mut *tx)
        .await
    {
        error!("failed to insert post {}: {}", &identifier, e);
        return;
    }

    tx.commit().await.unwrap();
}

async fn get_or_create_tag_id(
    db: &mut Transaction<'_, Postgres>,
    tag_name: &str,
) -> Result<i32, Error> {
    get_create_id(db, tag_name, GET_TAG_ID, CREATE_RETURN_TAG_ID).await
}

async fn get_or_create_category_id(
    db: &mut Transaction<'_, Postgres>,
    category_name: &str,
) -> Result<i32, Error> {
    get_create_id(
        db,
        category_name,
        GET_CATEGORY_ID,
        CREATE_RETURN_CATEGORY_ID,
    )
    .await
}

async fn get_create_id(
    db: &mut Transaction<'_, Postgres>,
    field_name: &str,
    fetch_query: &str,
    create_fetch_query: &str,
) -> Result<i32, Error> {
    if let Some(id) = query(fetch_query)
        .bind(field_name)
        .fetch_optional(&mut **db)
        .await?
    {
        Ok(id.get("id"))
    } else {
        let id = query(create_fetch_query)
            .bind(field_name)
            .fetch_one(&mut **db)
            .await?
            .get("id");
        Ok(id)
    }
}
