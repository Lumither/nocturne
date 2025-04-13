use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid front-matter: {}", msg)]
    InvalidFrontMatter { msg: String },

    #[error("fs error: {}", msg)]
    FSError { msg: String },
}
