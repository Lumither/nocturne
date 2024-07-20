use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostIdxError {
    #[error("Failed to parse UUID <{}>: {}", id, err_msg)]
    InvalidUUID { id: String, err_msg: String },

    #[error(
        "Failed to parse `last update time` for <{}> when creating post index: {}",
        id,
        err_msg
    )]
    InvalidLastUpdate { id: String, err_msg: String },

    #[error(
        "Failed to parse `first update time` for <{}> when creating post index: {}",
        id,
        err_msg
    )]
    InvalidFirstUpdate { id: String, err_msg: String },

    #[error(
        "Failed to parse `tag` for <{}> when creating post index: <{}>: {}",
        id,
        invalid_string,
        err_msg
    )]
    InvalidTagString {
        id: String,
        invalid_string: String,
        err_msg: String,
    },

    #[error(
        "Database error on writing `{}` to table `{}`: <{}>: {}",
        data_desc,
        db_table,
        id,
        err_msg
    )]
    DBWriteFailure {
        data_desc: String,
        db_table: String,
        id: String,
        err_msg: String,
    },

    #[error(
        "Database error on Reading `{}` to table `{}`: <{}>: {}",
        data_desc,
        db_table,
        id,
        err_msg
    )]
    DBReadFailure {
        data_desc: String,
        db_table: String,
        id: String,
        err_msg: String,
    },
}
