WITH filtered_posts AS
         (SELECT p.id, p.date_created
          FROM posts p
          WHERE $1::text[] IS NULL
             OR (
              CASE
                  WHEN $4 THEN (SELECT COUNT(DISTINCT t.name)
                                FROM post_tag pt
                                         JOIN tags t ON pt.tag = t.id
                                WHERE pt.post = p.id
                                  AND t.name = ANY ($1::text[])) = cardinality($1)
                  ELSE EXISTS (SELECT 1
                               FROM post_tag pt
                                        JOIN tags t ON pt.tag = t.id
                               WHERE pt.post = p.id
                                 AND t.name = ANY ($1::text[]))
                  END
              ))
SELECT COUNT(*) AS total_count
FROM filtered_posts;
