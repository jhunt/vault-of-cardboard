FROM rust:1.42 AS build

WORKDIR /src
COPY . .
RUN cargo install --locked --path . --root /vcb

FROM ubuntu:18.04
RUN apt-get update \
 && apt-get install -y libpq5 \
 && rm -rf /var/lib/apt/lists/*

COPY --from=build /vcb/bin/cardboard /cardboard
ENTRYPOINT ["/cardboard"]
# vim:ft=Dockerfile
