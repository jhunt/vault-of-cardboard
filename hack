#!/bin/sh
set -eu

if [ -z "${1:-}" ]; then
  echo >&2 "USAGE: $0 (up|down|ingest) [options...]"
  exit 1
fi

case $1 in
up|down|restart|build|recreate)
  exec docker-compose -f dev-env.yml -p vcbdev "$@"
  ;;
ingest)
  shift
  docker-compose -f dev-env.yml -p vcbdev \
         exec -e CACHE=/cache api \
              /usr/bin/ingest "$@"
  docker-compose -f dev-env.yml -p vcbdev \
         exec api \
              cardboard rescry -r /cache/dat \
                               -c /data/cards.json \
                               -p /data/prices.json \
                               -l /data/lookup.json
  exit 0
  ;;
*)
  echo >&2 "Unrecognized command \`$1'"
  echo >&2 "USAGE: $0 (up|down|ingest) [options...]"
  exit 2
  ;;
esac

exit 3
