#!/bin/bash
set -eu

ARGS=()
if [[ -n "${1:-}" ]]; then
  if [[ ${1:0:1} = '-' ]]; then
    ARGS="$@"
    set --
  elif [[ ${1:-} == "squid" ]]; then
    ARGS="${@:2}"
    set --
  fi
fi

if [[ -z "${1:-}" ]]; then
  mkdir -p /logs /cache
  if [[ ! -d /cache/00 ]]; then
    echo "initializing squid cache for the first time"
    squid -N -f /etc/squid/squid.conf -z
  fi
  exec squid -f /etc/squid/squid.conf -NYCd 1 ${ARGS[@]}
else
  exec "$@"
fi
