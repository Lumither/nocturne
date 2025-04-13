CREATE TABLE tags
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(20) UNIQUE NOT NULL,
    description TEXT DEFAULT ''
);

CREATE TABLE categories
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(20) UNIQUE NOT NULL,
    description TEXT DEFAULT ''
);

CREATE TABLE post_status
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL
);

INSERT INTO post_status (name)
VALUES ('draft'),
       ('published'),
       ('planned'),
       ('hidden');

CREATE TABLE posts
(
    id           UUID PRIMARY KEY,
    identifier   VARCHAR(100) UNIQUE                           NOT NULL,
    title        VARCHAR(255)                                  NOT NULL,
    subtitle     VARCHAR(255)                                  NOT NULL,
    date_created DATE                                          NOT NULL,
    date_updated DATE,
    status       INTEGER REFERENCES post_status (id) DEFAULT 1 NOT NULL,
    content      TEXT                                          NOT NULL,
    category     INTEGER REFERENCES categories (id)            NOT NULL
);

CREATE TABLE post_tag
(
    post UUID REFERENCES posts (id) ON DELETE CASCADE,
    tag  INTEGER REFERENCES tags (id) ON DELETE CASCADE,

    CONSTRAINT post_tag_pk PRIMARY KEY (post, tag)
);

CREATE TABLE metadata
(
    pid        UUID REFERENCES posts (id) ON DELETE CASCADE NOT NULL,
    meta_key   VARCHAR(50)                                  NOT NULL,
    meta_value TEXT                                         NOT NULL,

    CONSTRAINT metadata_pk PRIMARY KEY (pid, meta_key)
);

-- index
CREATE INDEX idx_tags_name ON tags (name);
CREATE INDEX idx_categories_name ON categories (name);
CREATE INDEX idx_posts_identifier ON posts (identifier);
CREATE INDEX idx_posts_status ON posts (status);
CREATE INDEX idx_posts_category ON posts (category);
CREATE INDEX idx_post_tag_post ON post_tag (post);
CREATE INDEX idx_post_tag_tag ON post_tag (tag);
CREATE INDEX idx_metadata_pid ON metadata (pid);