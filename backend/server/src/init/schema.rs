use sqlx::{query, Pool, Postgres};
use tracing::info;
//
// const POST_BASE_MODEL: &str = r##"
// CREATE TABLE IF NOT EXISTS Post
// (
//     post_id      UUID PRIMARY KEY,
//     title        VARCHAR(255) NOT NULL,
//     category     VARCHAR(255) NOT NULL,
//     filename     TEXT UNIQUE  NOT NULL,
//     summary      TEXT,
//     sub_title    TEXT,
//     content      TEXT         NOT NULL,
//     last_update  TIMESTAMPTZ,
//     first_update TIMESTAMPTZ  NOT NULL
// );
// "##;
//
// // base model, will add more cols at runtime
// const POST_META_MODEL: &str = r##"
// CREATE TABLE IF NOT EXISTS Meta
// (
//     post_id      UUID PRIMARY KEY
// );
// "##;
//
// const POST_HASH_MODEL: &str = r##"
// CREATE TABLE IF NOT EXISTS Hash
// (
//     post_id      UUID PRIMARY KEY,
//     hash         VARCHAR(44)
// );
// "##;
//
// const TAG_MODEL: &str = r##"
// CREATE TABLE IF NOT EXISTS Tag
// (
//     post_id UUID         NOT NULL,
//     tag     VARCHAR(255) NOT NULL
// );
// "##;

pub async fn init(db_connection: &Pool<Postgres>) {
    // query(POST_BASE_MODEL).execute(db_connection).await.unwrap();
    // query(POST_META_MODEL).execute(db_connection).await.unwrap();
    // query(TAG_MODEL).execute(db_connection).await.unwrap();
    // query(POST_HASH_MODEL).execute(db_connection).await.unwrap();
    info!("database schema initialized");
}
