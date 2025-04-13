INSERT INTO categories(name)
VALUES ($1)
RETURNING id;
