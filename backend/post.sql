CREATE TABLE IF NOT EXISTS Post
(
    post_id      UUID PRIMARY KEY,
    title        VARCHAR(255) NOT NULL,
    summary      TEXT         NOT NULL,
    content      TEXT         NOT NULL,
    last_update  TIMESTAMP,
    first_update TIMESTAMP    NOT NULL
);

drop table if exists post;

INSERT INTO Post (post_id, title, summary, content, last_update, first_update)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (post_id)
    DO UPDATE SET title        = excluded.title,
                  summary      = excluded.summary,
                  content      = excluded.content,
                  last_update  = excluded.last_update,
                  first_update = excluded.first_update;

SELECT *
FROM Post
WHERE post_id = $1;
