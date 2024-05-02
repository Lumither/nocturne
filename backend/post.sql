CREATE TABLE IF NOT EXISTS Post
(
    post_id     UUID PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    content     TEXT         NOT NULL,
    last_update TIMESTAMP
);

INSERT INTO Post (post_id, title, content, last_update)
VALUES (?1, ?2, ?3, ?4)
ON CONFLICT (post_id)
    DO UPDATE SET title       = excluded.title,
                  content     = excluded.content,
                  last_update = excluded.last_update
