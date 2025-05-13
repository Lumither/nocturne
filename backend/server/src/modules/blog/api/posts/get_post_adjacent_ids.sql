WITH target_create_date AS (SELECT p.date_created AS date FROM posts p WHERE p.id = $1)
SELECT (SELECT p.id
        FROM posts p
        WHERE p.date_created < t.date
        ORDER BY p.date_created DESC
        LIMIT 1) AS prev,
       (SELECT p.id
        FROM posts p
        WHERE p.date_created > t.date
        ORDER BY p.date_created
        LIMIT 1) AS next
FROM target_create_date t

