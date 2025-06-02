INSERT INTO posts
(id, identifier, title, subtitle, date_created, date_updated, status, content, hash, category)
VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, 'draft'), $8, $9, $10);
