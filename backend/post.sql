CREATE TABLE IF NOT EXISTS Post
(
    post_id     UUID PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    summary     TEXT         NOT NULL,
    content     TEXT         NOT NULL,
    last_update TIMESTAMP
);

drop table if exists post;

INSERT INTO Post (post_id, title, summary, content, last_update)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (post_id)
    DO UPDATE SET title       = excluded.title,
                  summary     = excluded.summary,
                  content     = excluded.content,
                  last_update = excluded.last_update
