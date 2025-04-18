mod checker;
mod error;
mod utils;

use std::{path::PathBuf, sync::Arc};

use crate::{
    modules::blog::components::{
        check_update::apply::apply_deltas, consistency_guard::checker::compare_db_local_post_hash,
        static_rsc::GIT_WORK_DIR,
    },
    scheduler::task_func::AsyncTaskFunc,
};

use sqlx::PgPool;
use tracing::{error, trace};

struct Args {
    db_connection: PgPool,
    git_work_dir: PathBuf,
}

pub fn task(db_connection: PgPool) -> impl AsyncTaskFunc {
    let args = Arc::new(Args {
        db_connection,
        git_work_dir: GIT_WORK_DIR.clone(),
    });
    move || Box::pin(workflow(args.clone()))
}

async fn workflow(args: Arc<Args>) {
    let db = &args.db_connection;
    let git_work_dir = &args.git_work_dir;

    let changes = match compare_db_local_post_hash(db, git_work_dir).await {
        Ok(c) => c,
        Err(e) => {
            error!("failed to compare db and local post list: {}", e);
            return;
        }
    };
    apply_deltas(db, changes).await;
    trace!("local-database consistency checked");
}
