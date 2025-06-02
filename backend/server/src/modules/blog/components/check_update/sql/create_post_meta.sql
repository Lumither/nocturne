INSERT INTO metadata (pid, meta_key, meta_value)
SELECT $1, key, value
FROM UNNEST($2, $3) AS k(key, value);
