CREATE TABLE IF NOT EXISTS Meta
(
    post_id UUID PRIMARY KEY
);

DROP table meta;

DO
$$
    DECLARE
        _post_id        UUID   := '{}';
        _meta_cols      JSONB  := '{}';
        _exists         BOOLEAN;
        _col_name       TEXT;
        _col_value      TEXT;
        _sql            TEXT;
        _insert_columns TEXT   := 'post_id';
        _insert_values  TEXT   := quote_literal(_post_id);
        _update_set     TEXT   := '';
        _blacklist      TEXT[] := ARRAY ['id', 'tags'];
    BEGIN
        SELECT EXISTS(SELECT 1 FROM meta WHERE post_id = _post_id) INTO _exists;

        FOR _col_name, _col_value IN SELECT key, value FROM jsonb_each_text(_meta_cols)
            LOOP
                IF _col_name = ANY (_blacklist) THEN
                    CONTINUE;
                END IF;

                IF NOT EXISTS (SELECT 1
                               FROM information_schema.columns
                               WHERE table_name = 'meta'
                                 AND column_name = _col_name
                                 -- may change in the future
                                 AND table_schema = 'public') THEN
                    _sql := 'ALTER TABLE meta ADD COLUMN ' || quote_ident(_col_name) || ' TEXT';
                    EXECUTE _sql;
                END IF;
                _insert_columns := _insert_columns || ', ' || quote_ident(_col_name);
                _insert_values := _insert_values || ', ' || quote_literal(_col_value);
                _update_set := _update_set || ', ' || quote_ident(_col_name) || ' = ' || quote_literal(_col_value);
            END LOOP;

        _update_set := substring(_update_set FROM 3);

        IF _exists THEN
            _sql := 'UPDATE meta SET ' || _update_set || ' WHERE post_id = ' || quote_literal(_post_id);
            EXECUTE _sql;
        ELSE
            _sql := 'INSERT INTO meta (' || _insert_columns || ') VALUES (' || _insert_values || ')';
            EXECUTE _sql;
        END IF;
    END;
$$ LANGUAGE plpgsql;

SELECT jsonb_strip_nulls(to_jsonb(t)) AS meta
FROM meta t
WHERE post_id = '8ca3fa2b-f040-448c-8885-81aab56db9bb';

WITH target_post AS (SELECT t.date FROM meta t WHERE post_id = '8ca3fa2b-f040-448c-8885-81aab56db9bb'),
     prev AS (SELECT post_id, t.title, t.sub_title, t.header_img
              FROM meta t
              WHERE t.date < (SELECT date FROM target_post)
              ORDER BY t.date DESC
              LIMIT 1),
     next AS (SELECT post_id, t.title, t.sub_title, t.header_img
              FROM meta t
              WHERE t.date > (SELECT date FROM target_post)
              ORDER BY t.date
              LIMIT 1)
SELECT json_strip_nulls(json_build_object(
        'prev',
        (SELECT json_build_object('id', post_id, 'title', title, 'sub_title', sub_title, 'header_img', header_img)
         FROM prev),
        'next',
        (SELECT json_build_object('id', post_id, 'title', title, 'sub_title', sub_title, 'header_img', header_img)
         FROM next)
                        )) AS result;

