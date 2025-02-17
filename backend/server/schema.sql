CREATE TABLE tag
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(20) UNIQUE NOT NULL,
    description TEXT
);

CREATE TABLE category
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(20) UNIQUE NOT NULL,
    description TEXT
);

CREATE TABLE post
(
    id             SERIAL PRIMARY KEY,
    identifier     VARCHAR(50) UNIQUE              NOT NULL,
    title          VARCHAR(255)                    NOT NULL,
    subtitle       VARCHAR(255)                    NOT NULL,
    published_date DATE                            NOT NULL,
    last_update    DATE,
    content        TEXT                            NOT NULL,

    category       SERIAL REFERENCES category (id) NOT NULL
);

CREATE TABLE post_tag
(
    post SERIAL REFERENCES post (id),
    tag  SERIAL REFERENCES tag (id)
)