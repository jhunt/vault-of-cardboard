#!/bin/sh

exec docker-compose -f dev-env.yml -p vcbdev "$@"
