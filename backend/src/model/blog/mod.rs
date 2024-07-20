pub const POST_BASE_MODEL: &str = r##"
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
"##;

// base mode, will add more cols in runtime
pub const POST_META_MODEL: &str = r##"
CREATE TABLE IF NOT EXISTS Meta
(
    post_id      UUID PRIMARY KEY
);
"##;

pub const POST_HASH_MODEL: &str = r##"
CREATE TABLE IF NOT EXISTS Hash
(
    post_id      UUID PRIMARY KEY,
    hash         VARCHAR(44)
);
"##;

pub const TAG_MODEL: &str = r##"
CREATE TABLE IF NOT EXISTS Tag
(
    post_id UUID         NOT NULL,
    tag     VARCHAR(255) NOT NULL
);
"##;
