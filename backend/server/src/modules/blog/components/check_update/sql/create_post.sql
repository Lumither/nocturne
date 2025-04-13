INSERT INTO posts
(id, identifier, title, subtitle, date_created, date_updated, status, content, category)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);

INSERT INTO post_tag (post, tag)
SELECT $1, tag
FROM UNNEST($10) AS t(tag);
