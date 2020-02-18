#!/bin/sh
set -eu

for ext in uuid-osp; do
  echo "loading extension '$ext'..."
  psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" \ -d "$POSTGRES_DB" \
       -c "CREATE EXTENSION IF NOT EXISTS \"$ext\";"
done
echo "extensions loaded."
