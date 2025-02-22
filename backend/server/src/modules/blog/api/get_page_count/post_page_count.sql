SELECT cast(ceil(count(*) / cast($1 AS FLOAT)) AS INTEGER) AS count
FROM posts;
