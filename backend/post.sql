CREATE TABLE Post
(
    post_id     UUID PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    content     TEXT         NOT NULL,
    last_update TIMESTAMP
);
