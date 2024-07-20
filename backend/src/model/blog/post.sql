CREATE TABLE IF NOT EXISTS Post
(
    post_id      UUID PRIMARY KEY,
    title        VARCHAR(255) NOT NULL,
    category     VARCHAR(255) NOT NULL,
    header_img   TEXT,
    summary      TEXT,
    sub_title    TEXT,
    content      TEXT         NOT NULL,
    last_update  TIMESTAMPTZ  NOT NULL,
    first_update TIMESTAMPTZ  NOT NULL
);
