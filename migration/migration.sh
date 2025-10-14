#!/bin/sh
set -e

PGPASSFILE="/root/.pgpass"
echo "postgres:5432:${POSTGRES_DB}:${POSTGRES_USER}:${POSTGRES_PASSWORD}" > "$PGPASSFILE"
chmod 600 "$PGPASSFILE"

echo "waiting for postgres"
until pg_isready -h postgres -U "$POSTGRES_USER" -d "$POSTGRES_DB"; do
   echo "retrying, sleep 2s"
   sleep 2
done
echo "db ready, running migrations"
for file in /migrations/*.sql; do
  echo "applying $file"
  psql -h postgres -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f "$file"
done
echo "migration complete"
