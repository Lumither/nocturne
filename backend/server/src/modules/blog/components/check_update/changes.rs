use git2::Delta;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug)]
pub enum Change {
    DELETE {
        uuid: Uuid,
    },
    RENAME {
        uuid: Uuid,
        from: PathBuf,
        to: PathBuf,
    },
    UPDATE {
        uuid: Uuid,
        content: String,
    },
    CREATE {
        uuid: Uuid,
        path: PathBuf,
        content: String,
    },
}

#[derive(Debug, Clone)]
pub struct DeltaIdx {
    pub delta_type: Delta,
    pub content: String,
}
