SELECT p.id
FROM posts p
WHERE $1::text[] IS NULL
   OR (
    CASE
        WHEN $4 THEN (SELECT COUNT(DISTINCT t.name)
                      FROM post_tag pt
                               JOIN tags t ON pt.tag = t.id
                      WHERE pt.post = p.id
                        AND t.name = ANY ($1::text[]))
            = cardinality($1)
        ELSE
            EXISTS (SELECT 1
                    FROM post_tag pt
                             JOIN tags t ON pt.tag = t.id
                    WHERE pt.post = p.id
                      AND t.name = ANY ($1::text[]))
        END
    )
ORDER BY p.date_created DESC
LIMIT $2 OFFSET $3;
