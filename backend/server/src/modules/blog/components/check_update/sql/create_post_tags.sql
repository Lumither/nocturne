INSERT INTO post_tag (post, tag)
SELECT $1, tag
FROM UNNEST($2) AS t(tag);
