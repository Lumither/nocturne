SELECT p.id,
       identifier,
       title,
       subtitle,
       date_created,
       date_updated,
       c.name                            AS category,
       (SELECT array_agg(t.name)
        FROM tags t
                 JOIN post_tag pt ON pt.tag = t.id
        WHERE pt.post = p.id)            AS tags,
       (SELECT m.meta_value
        FROM metadata m
        WHERE m.pid = p.id
          AND m.meta_key = 'header_img') AS header_img
FROM posts p
         JOIN public.categories c on c.id = p.category
WHERE p.id = ANY ($1)
ORDER BY p.date_created DESC;