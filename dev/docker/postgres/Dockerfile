FROM postgres:11

RUN apt-get update \
 && apt-get install -y --no-install-recommends postgresql-contrib-$PG_MAJOR \
 && rm -rf /var/lib/apt/lists/*

ADD install-extensions.sh /docker-entrypoint-initdb.d/
