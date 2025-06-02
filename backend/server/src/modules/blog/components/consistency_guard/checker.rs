use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::modules::blog::components::{
    check_update::{
        changes::{Change, Create, Delete, Update},
        utils::extract_post_id,
    },
    consistency_guard::{error::Error, utils::rec_find_dir_entries},
    static_rsc::BLOG_POST_PATH_PATTERN,
};
use markdown::MdFile;

use sqlx::{FromRow, PgPool, query_as};
use tracing::error;
use uuid::Uuid;

const FETCH_POST_HASH: &str = include_str!("sql/fetch_post_hash.sql");

#[derive(Debug, FromRow)]
struct PostIdHash {
    id: Uuid,
    hash: String,
}

struct PostPath {
    id: Uuid,
    path: PathBuf,
    payload: MdFile,
}

#[derive(Default)]
struct DeltaSlot {
    local: Option<PostPath>,
    db: Option<PostIdHash>,
}

impl DeltaSlot {
    pub fn to_change(&self) -> Option<Change> {
        match (&self.local, &self.db) {
            (Some(local), Some(db)) => {
                if local.payload.hash == db.hash {
                    None
                } else {
                    Some(Change::Update(Update {
                        uuid: local.id,
                        path: local.path.clone(),
                        payload: local.payload.clone(),
                    }))
                }
            }
            (Some(local), None) => Some(Change::Create(Create {
                uuid: local.id,
                path: local.path.clone(),
                payload: local.payload.clone(),
            })),
            (None, Some(db)) => Some(Change::Delete(Delete {
                uuid: db.id,
                path: None,
            })),
            (None, None) => None,
        }
    }
}

pub async fn compare_db_local_post_hash(
    db: &PgPool,
    git_work_dir: &Path,
) -> Result<Vec<Change>, Error> {
    let db_post_hashes = query_as::<_, PostIdHash>(FETCH_POST_HASH)
        .fetch_all(db)
        .await?;
    let local_posts = fetch_local_posts(git_work_dir)?;
    calculate_post_changes(local_posts, db_post_hashes)
}

fn fetch_local_posts(git_work_dir: &Path) -> Result<Vec<PostPath>, Error> {
    let mut post_hashes = vec![];
    let entries = rec_find_dir_entries(&git_work_dir.join("posts"))?;

    for entry in entries {
        let relative_pth = entry.strip_prefix(git_work_dir).unwrap();
        if !BLOG_POST_PATH_PATTERN.is_match(relative_pth.to_str().unwrap()) {
            continue;
        }
        let post_str = fs::read_to_string(&entry)?;
        let post = MdFile::from_str(&post_str)?;
        let post_id = match extract_post_id(&post) {
            None => {
                error!(
                    "failed to parse uuid for file {}, skipping",
                    &entry.display()
                );
                continue;
            }
            Some(v) => v,
        };
        post_hashes.push(PostPath {
            id: post_id,
            path: entry,
            payload: post,
        });
    }

    Ok(post_hashes)
}

fn calculate_post_changes(local: Vec<PostPath>, db: Vec<PostIdHash>) -> Result<Vec<Change>, Error> {
    let mut slots: HashMap<Uuid, DeltaSlot> = HashMap::new();
    for local_post in local {
        let id = local_post.id;
        slots.entry(id).or_default().local = Some(local_post);
    }
    for db_post in db {
        let id = db_post.id;
        slots.entry(id).or_default().db = Some(db_post);
    }

    let mut changes = vec![];
    for slot in slots.values() {
        if let Some(change) = slot.to_change() {
            changes.push(change);
        }
    }
    Ok(changes)
}
