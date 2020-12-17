# vim:ft=Dockerfile
FROM rust:1.42 AS build

WORKDIR /src
COPY . .
RUN cargo install --locked --path . --root /vcb

FROM ubuntu:20.04
RUN apt-get update \
 && apt-get install -y libpq5 curl ca-certificates jq \
 && rm -rf /var/lib/apt/lists/*

RUN curl -sLo /usr/bin/s3 https://github.com/jhunt/s3/releases/download/v0.3.0/s3-linux-amd64 \
 && chmod 0755 /usr/bin/s3

COPY --from=build /vcb/bin/cardboard /usr/bin/cardboard
COPY ingest /usr/bin/ingest
