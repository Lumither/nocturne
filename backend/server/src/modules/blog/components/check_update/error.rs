use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostIdxError {
    #[error("missing essential field `{}` for file `{}`", field, filename)]
    MissingField { field: String, filename: String },

    #[error(
        "invalid field `{}` for file `{}`, <{}>: {}",
        field_name,
        filename,
        err_field,
        msg
    )]
    InvalidField {
        field_name: String,
        err_field: String,
        filename: String,
        msg: String,
    },

    #[error(
        "database error on writing `{}` to table `{}`: <{}>: {}",
        data_desc,
        db_table,
        id,
        msg
    )]
    DBWriteFailure {
        data_desc: String,
        db_table: String,
        id: String,
        msg: String,
    },

    #[error(
        "Database error on Reading `{}` from table `{}`: <{}>: {}",
        data_desc,
        db_table,
        id,
        msg
    )]
    DBReadFailure {
        data_desc: String,
        db_table: String,
        id: String,
        msg: String,
    },
}
