CREATE TABLE IF NOT EXISTS Meta
(
    post_id UUID PRIMARY KEY
);

DROP table meta;

DO
$$
    DECLARE
        _post_id        UUID  := 'd5a209d4-93b1-474e-8e4e-9c78d6e6e763'; -- test data
        _meta_cols      JSONB := '{"title": " title", "tags": "example tags", "new_col1": "new_value1", "new_col3": "new_value2"}'; -- test data
        _exists         BOOLEAN;
        _col_name       TEXT;
        _col_value      TEXT;
        _sql            TEXT;
        _insert_columns TEXT  := 'post_id';
        _insert_values  TEXT  := quote_literal(_post_id);
        _update_set     TEXT  := '';
    BEGIN
        SELECT EXISTS(SELECT 1 FROM meta WHERE post_id = _post_id) INTO _exists;

        FOR _col_name, _col_value IN SELECT key, value FROM jsonb_each_text(_meta_cols)
            LOOP
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
