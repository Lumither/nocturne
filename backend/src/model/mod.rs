pub mod post;

pub const POST_DB_MODEL: &str = r##"
CREATE TABLE IF NOT EXISTS Post
(
    post_id      UUID PRIMARY KEY,
    title        VARCHAR(255) NOT NULL,
    summary      TEXT,
    sub_title    TEXT,
    content      TEXT         NOT NULL,
    last_update  TIMESTAMPTZ  NOT NULL,
    first_update TIMESTAMPTZ  NOT NULL
);
"##;

pub const TAG_DB_MODEL: &str = r##"
CREATE TABLE IF NOT EXISTS Tag
(
    post_id UUID         NOT NULL,
    tag     VARCHAR(255) NOT NULL
);
"##;
