use crate::modules::blog::components::check_update::changes::{
    Change, Create, Delete, Move, Update,
};

use futures::{
    FutureExt,
    future::{BoxFuture, join_all},
};
use sqlx::PgPool;

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
    todo!()
}

async fn handle_move(db: &PgPool, mv: Move) {
    todo!()
}

async fn handle_update(db: &PgPool, update: Update) {
    todo!()
}

async fn handle_create(db: &PgPool, create: Create) {
    todo!()
}
