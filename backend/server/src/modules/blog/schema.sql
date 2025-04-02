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

CREATE TABLE posts
(
    id             UUID UNIQUE                       NOT NULL,
    identifier     VARCHAR(100) UNIQUE               NOT NULL,
    title          VARCHAR(255)                      NOT NULL,
    subtitle       VARCHAR(255)                      NOT NULL,
    published_date DATE                              NOT NULL,
    last_update    DATE,
    content        TEXT                              NOT NULL,

    category       SERIAL REFERENCES categories (id) NOT NULL
);

CREATE TABLE post_tag
(
    post UUID REFERENCES posts (id),
    tag  SERIAL REFERENCES tags (id)
);

CREATE TABLE metadata
(
    pid        UUID REFERENCES posts (id) NOT NULL,
    meta_key   VARCHAR(50)                NOT NULL,
    meta_value TEXT                       NOT NULL,

    CONSTRAINT meta_pk PRIMARY KEY (pid, meta_key)
);
