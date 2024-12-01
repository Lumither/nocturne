use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("missing front-matter field: id")]
    MissingID,

    #[error("invalid file id: {}", msg)]
    InvalidID { msg: String },

    #[error("invalid front-matter: {}", msg)]
    InvalidFrontMatter { msg: String },

    #[error("fs error: {}", msg)]
    FSError { msg: String },
}
