use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("db error")]
    Db(#[from] sqlx::Error),

    #[error("io error")]
    Io(#[from] io::Error),

    #[error("invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("invalid md file")]
    MdFile(#[from] markdown::error::Error),
}
