use std::error::Error;
use std::path::Path;

use crate::blog::cron::check_update::utils::get_md_file_basename;
use crate::blog::cron::check_update::{error::PostIdxError, utils::parse_date};
use markdown::MdFile;

use sqlx::{query, PgPool, Pool, Postgres, Row};
use tokio::runtime::Runtime;
use tracing::{info, warn};

async fn base(db_connection: &PgPool, post: &MdFile) -> Result<(), PostIdxError> {
    let post_id = post.file_id;
    let title = match post.meta.get("title") {
        None => {
            return Err(PostIdxError::MissingField {
                field: "title".to_string(),
                filename: post.filename.clone(),
            })
        }
        Some(title) => title.as_str().unwrap_or_else(|| {
            warn!("invalid title for `{}`", &post.filename);
            "invalid title"
        }),
    };
    let sub_title = post
        .meta
        .get("sub_title")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let summary = post
        .meta
        .get("summary")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let content = post.content.as_str();
    let category = match post.meta.get("category") {
        None => {
            return Err(PostIdxError::MissingField {
                field: "category".to_string(),
                filename: post.filename.clone(),
            })
        }
        Some(title) => title.as_str().unwrap_or_else(|| {
            warn!("invalid category for `{}`", &post.filename);
            "invalid category"
        }),
    };
    let last_update = {
        if let Some(last_update) = post.meta.get("last_update") {
            match parse_date(last_update.as_str().unwrap()) {
                Ok(date) => Some(date),
                Err(e) => {
                    return Err(PostIdxError::InvalidField {
                        field_name: "last_update".to_string(),
                        err_field: last_update.to_string(),
                        filename: post.filename.clone(),
                        msg: e.to_string(),
                    })
                }
            }
        } else {
            None
        }
    };
    let first_update = match post.meta.get("date") {
        None => {
            return Err(PostIdxError::MissingField {
                field: "date".to_string(),
                filename: post.filename.clone(),
            })
        }
        Some(first_update) => match parse_date(first_update.as_str().unwrap()) {
            Ok(date) => date,
            Err(e) => {
                return Err(PostIdxError::InvalidField {
                    field_name: "date".to_string(),
                    err_field: first_update.to_string(),
                    filename: post.filename.clone(),
                    msg: e.to_string(),
                })
            }
        },
    };
    let filename = get_md_file_basename(&post.filename);

    if let Err(e) = query(
        r##"
INSERT INTO Post (post_id, title, summary, content, last_update, first_update, sub_title, category, filename)
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
    filename = EXCLUDED.filename;
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
        .bind(filename)
        .execute(db_connection)
        .await
    {
        return Err(
            PostIdxError::DBWriteFailure {
                data_desc: "post base information".to_string(),
                db_table: "Post".to_string(),
                id: post_id.to_string(),
                msg: e.to_string(),
            }
        );
    }
    Ok(())
}

async fn tag(db_connection: &PgPool, post: &MdFile) -> Result<(), PostIdxError> {
    if let Some(tags_string) = post.meta.get("tags").to_owned() {
        let tags = match tags_string.as_array().map(|vec| {
            vec.iter()
                .map(|val| val.as_str().unwrap())
                .collect::<Vec<_>>()
        }) {
            None => {
                return Err(PostIdxError::MissingField {
                    field: "tags".to_string(),
                    filename: post.filename.clone(),
                })
            }
            Some(val) => val,
        };

        let post_id = post.file_id;

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
                msg: e.to_string(),
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
                        msg: e.to_string(),
                    })
                }
            }
        }
    }

    Ok(())
}

async fn hash(db_connection: &PgPool, post: &MdFile) -> Result<bool, PostIdxError> {
    let base64_form = post.hash.clone();
    match query("SELECT hash FROM Hash WHERE post_id = $1")
        .bind(post.file_id)
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
                .bind(post.file_id)
                .bind(&base64_form)
                .execute(db_connection)
                .await
                {
                    return Err(PostIdxError::DBWriteFailure {
                        data_desc: "post hash".to_string(),
                        db_table: "Hash".to_string(),
                        id: post.file_id.to_string(),
                        msg: e.to_string(),
                    });
                }
                Ok(true)
            }
        }
        Err(e) => Err(PostIdxError::DBReadFailure {
            data_desc: "post hash".to_string(),
            db_table: "Hash".to_string(),
            id: post.file_id.to_string(),
            msg: e.to_string(),
        }),
    }
}

async fn meta(db_connection: &PgPool, post: &MdFile) -> Result<(), PostIdxError> {
    if let Err(e) = query(
        // todo: sanitization
        format!(
            r#"
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
        _blacklist      TEXT[] := ARRAY['tags'];
    BEGIN
        SELECT EXISTS(SELECT 1 FROM meta WHERE post_id = _post_id) INTO _exists;

        FOR _col_name, _col_value IN SELECT key, value FROM jsonb_each_text(_meta_cols)
            LOOP
                IF _col_name = ANY(_blacklist) THEN
                    CONTINUE;
                END IF;

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
        "#, post.file_id.to_string().as_str(), post.meta
        ).as_str()
    )
        .execute(db_connection)
        .await {
        Err(PostIdxError::DBWriteFailure {
            data_desc: "post meta data".to_string(),
            db_table: "Meta".to_string(),
            id: post.file_id.to_string(),
            msg: e.to_string(),
        })
    } else {
        Ok(())
    }
}

pub async fn index_file(path: &Path, db_conn: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let md_file = MdFile::from_file(path)?;

    async fn trail(db_conn: &Pool<Postgres>, md_file: &MdFile) -> Result<(), PostIdxError> {
        base(db_conn, md_file).await?;
        tag(db_conn, md_file).await?;
        meta(db_conn, md_file).await?;
        hash(db_conn, md_file).await.map(|_| ())
    }
    let mut cnt = 5;

    while let Err(e) = trail(db_conn, &md_file).await {
        match e {
            PostIdxError::DBWriteFailure { .. } => {
                cnt -= 1;
            }
            PostIdxError::DBReadFailure { .. } => {
                cnt -= 1;
            }
            _ => return Err(e.into()),
        }
        if cnt <= 0 {
            return Err(e.into());
        }
    }

    info!("indexed markdown file: {}", path.display());
    Ok(())
}

pub fn drop_index(file_basename: &str, db_conn: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = query(
            r#"
WITH deleted_post AS (
    DELETE FROM post
    WHERE id = (SELECT id FROM post WHERE filename = $1)
    RETURNING id
)
DELETE FROM tag WHERE post_id IN (SELECT id FROM deleted_post);
DELETE FROM meta WHERE post_id IN (SELECT id FROM deleted_post);
DELETE FROM hash WHERE post_id IN (SELECT id FROM deleted_post);
    "#,
        )
        .bind(file_basename)
        .execute(db_conn)
        .await
        {
            Err(Box::from(
                format!("failed to drop index: {}", e).to_string(),
            ))
        } else {
            Ok(())
        }
    })
}
