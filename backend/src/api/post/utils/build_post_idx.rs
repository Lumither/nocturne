use base64::Engine;
use base64::engine::general_purpose;
use chrono::DateTime;
use serde_json::{Map, Value};
use sqlx::{PgPool, query, Row};
use uuid::Uuid;

use crate::api::post::errors::refresh::PostIdxError;
use crate::constants::GLOBAL_TIME_FORMAT;

pub async fn base(
    db_connection: &PgPool,
    post: &Map<String, Value>,
    post_id: Uuid,
    meta: &Value,
) -> Result<(), PostIdxError> {
    let title = post["title"].as_str();
    let summary = post
        .get("summary")
        .and_then(|summary| summary.as_str())
        .unwrap_or("");
    let content = post["content"].as_str();
    let last_update =
        match DateTime::parse_from_str(post["last_update"].as_str().unwrap(), GLOBAL_TIME_FORMAT) {
            Ok(time) => time,
            Err(e) => {
                return Err(PostIdxError::InvalidLastUpdate {
                    id: post_id.to_string(),
                    err_msg: e.to_string(),
                });
            }
        };
    let first_update = match DateTime::parse_from_str(
        post["first_update"].as_str().unwrap(),
        GLOBAL_TIME_FORMAT,
    ) {
        Ok(time) => time,
        Err(e) => {
            return Err(PostIdxError::InvalidFirstUpdate {
                id: post_id.to_string(),
                err_msg: e.to_string(),
            })
        }
    };
    let sub_title = meta
        .get("sub_title")
        .and_then(|sub_title| sub_title.as_str())
        .unwrap_or("");
    let category = meta
        .get("category")
        .and_then(|category| category.as_str())
        .unwrap_or("N/A");
    let header_img = meta
        .get("header_img")
        .and_then(|header_img| header_img.as_str())
        .unwrap_or("");

    match query(
        r##"
INSERT INTO Post (post_id, title, summary, content, last_update, first_update, sub_title, category, header_img)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (post_id) DO
UPDATE SET
    title = EXCLUDED.title,
    summary = EXCLUDED.summary,
    content = EXCLUDED.content,
    last_update = EXCLUDED.last_update,
    first_update = EXCLUDED.first_update,
    sub_title = EXCLUDED.sub_title,
    category = EXCLUDED.category,
    header_img = EXCLUDED.header_img;
        "##,
    )
        .bind(post_id)
        .bind(title)
        .bind(summary)
        .bind(content)
        .bind(last_update)
        .bind(first_update)
        .bind(sub_title)
        .bind(category)
        .bind(header_img)
        .execute(db_connection)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return Err(
                PostIdxError::DBWriteFailure {
                    data_desc: "post base information".to_string(),
                    db_table: "Post".to_string(),
                    id: post_id.to_string(),
                    err_msg: e.to_string()
                }
            )
        }
    }

    Ok(())
}

pub async fn tag(db_connection: &PgPool, post_id: Uuid, meta: &Value) -> Result<(), PostIdxError> {
    if let Some(tags_string) = meta.get("tags").to_owned() {
        // parse tags string
        let parsed: Value = match serde_json::from_str(tags_string.as_str().unwrap()) {
            Ok(value) => value,
            Err(e) => {
                return Err(PostIdxError::InvalidTagString {
                    id: post_id.to_string(),
                    invalid_string: tags_string.to_string().to_string(),
                    err_msg: e.to_string(),
                })
            }
        };
        let string_array = parsed.as_array().unwrap();
        let tags: Vec<String> = string_array
            .iter()
            .map(|value| value.as_str().unwrap().to_string())
            .collect();

        // remove old record
        if let Err(e) = query("DELETE FROM Tag WHERE post_id = $1")
            .bind(post_id)
            .execute(db_connection)
            .await
        {
            return Err(PostIdxError::DBWriteFailure {
                data_desc: "post tag information".to_string(),
                db_table: "Tag".to_string(),
                id: post_id.to_string(),
                err_msg: e.to_string(),
            });
        }

        // add new record
        for tag in tags {
            match query("INSERT INTO Tag VALUES ($1, $2);")
                .bind(post_id)
                .bind(tag)
                .execute(db_connection)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    return Err(PostIdxError::DBWriteFailure {
                        data_desc: "post tag information".to_string(),
                        db_table: "Tag".to_string(),
                        id: post_id.to_string(),
                        err_msg: e.to_string(),
                    })
                }
            }
        }
    }

    Ok(())
}

/// # Arguments
///
/// * `db_connection`: database connection
/// * `post_id`: UUID of the post
/// * `hash`: Hex String
///
/// # Returns
/// Result<bool, PostIdxError>
/// - `bool`: if the hash is different from database, return true, else false
pub async fn hash(
    db_connection: &PgPool,
    post_id: Uuid,
    hash: Vec<u8>,
) -> Result<bool, PostIdxError> {
    let base64_form = general_purpose::STANDARD.encode(hash);
    match query("SELECT hash FROM Hash WHERE post_id = $1")
        .bind(post_id)
        .fetch_optional(db_connection)
        .await
    {
        Ok(row) => {
            let db_record: String = match row {
                None => "".to_string(),
                Some(row) => row.try_get("hash").unwrap_or("".to_string()),
            };
            if base64_form == db_record {
                Ok(false)
            } else {
                if let Err(e) = query(
                    r#"
INSERT INTO Hash (post_id, hash) VALUES ($1, $2)
    ON CONFLICT (post_id) DO UPDATE SET hash = EXCLUDED.hash"#,
                )
                .bind(post_id)
                .bind(&base64_form)
                .execute(db_connection)
                .await
                {
                    return Err(PostIdxError::DBWriteFailure {
                        data_desc: "post hash".to_string(),
                        db_table: "Hash".to_string(),
                        id: post_id.to_string(),
                        err_msg: e.to_string(),
                    });
                }
                Ok(true)
            }
        }
        Err(e) => Err(PostIdxError::DBReadFailure {
            data_desc: "post hash".to_string(),
            db_table: "Hash".to_string(),
            id: post_id.to_string(),
            err_msg: e.to_string(),
        }),
    }
}

pub async fn meta(db_connection: &PgPool, post_id: Uuid, meta: &Value) -> Result<(), PostIdxError> {
    if let Err(e) = query(
format!(
        r"
DO
$$
    DECLARE
        _post_id        UUID  := '{}';
        _meta_cols      JSONB := '{}';
        _exists         BOOLEAN;
        _col_name       TEXT;
        _col_value      TEXT;
        _sql            TEXT;
        _insert_columns TEXT  := 'post_id';
        _insert_values  TEXT  := quote_literal(_post_id);
        _update_set     TEXT  := '';
    BEGIN
        SELECT EXISTS(SELECT 1 FROM meta WHERE post_id = _post_id) INTO _exists;

        FOR _col_name, _col_value IN SELECT key, value FROM jsonb_each_text(_meta_cols)
            LOOP
                IF NOT EXISTS (SELECT 1
                               FROM information_schema.columns
                               WHERE table_name = 'meta'
                                 AND column_name = _col_name
                                 -- may change in the future
                                 AND table_schema = 'public') THEN
                    _sql := 'ALTER TABLE meta ADD COLUMN ' || quote_ident(_col_name) || ' TEXT';
                    EXECUTE _sql;
                END IF;
                _insert_columns := _insert_columns || ', ' || quote_ident(_col_name);
                _insert_values := _insert_values || ', ' || quote_literal(_col_value);
                _update_set := _update_set || ', ' || quote_ident(_col_name) || ' = ' || quote_literal(_col_value);
            END LOOP;

        _update_set := substring(_update_set FROM 3);

        IF _exists THEN
            _sql := 'UPDATE meta SET ' || _update_set || ' WHERE post_id = ' || quote_literal(_post_id);
            EXECUTE _sql;
        ELSE
            _sql := 'INSERT INTO meta (' || _insert_columns || ') VALUES (' || _insert_values || ')';
            EXECUTE _sql;
        END IF;
    END;
$$ LANGUAGE plpgsql;
        ", post_id.to_string().as_str(), meta.to_string()
).as_str()
    )
        .execute(db_connection)
        .await {
        Err(PostIdxError::DBWriteFailure {
            data_desc: "post meta data".to_string(),
            db_table: "Meta".to_string(),
            id: post_id.to_string(),
            err_msg: e.to_string(),
        })
    } else {
        Ok(())
    }
}
