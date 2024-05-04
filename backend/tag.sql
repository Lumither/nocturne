CREATE TABLE IF NOT EXISTS Tag
(
    post_id UUID         NOT NULL,
    tag     VARCHAR(255) NOT NULL
);

INSERT INTO Tag
VALUES ($1, $2);

SELECT *
FROM Tag
WHERE post_id = $1;
