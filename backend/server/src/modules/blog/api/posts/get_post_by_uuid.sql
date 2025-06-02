SELECT p.id,
       p.identifier,
       p.title,
       p.subtitle,
       p.status,
       p.date_created,
       p.date_updated,
       c.name                 AS category,
       (SELECT array_agg(t.name)
        FROM tags t
                 JOIN post_tag pt ON pt.tag = t.id
        WHERE pt.post = p.id) AS tags,
       p.content,
       (SELECT json_object_agg(m.meta_key, m.meta_value)
        FROM metadata m
        WHERE m.pid = p.id)   AS metadata
FROM posts p
         JOIN categories c ON c.id = p.category
WHERE p.id = $1;
