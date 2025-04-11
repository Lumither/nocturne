use markdown::error::Error as MarkdownError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("markdown file error")]
    MdFile(#[from] MarkdownError),

    #[error("db error")]
    Db(#[from] sqlx::Error),

    #[error("git error")]
    Git2(#[from] git2::Error),

    #[error("git remote url mismatch")]
    GitUrlMismatch,
}
