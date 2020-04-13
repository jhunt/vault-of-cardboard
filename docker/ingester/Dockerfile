FROM alpine:3

RUN apk update \
 && apk add curl ca-certificates jq \
 && rm -rf /var/cache/apk/*

RUN mkdir /workspace
WORKDIR /workspace

ADD ingest /bin/ingest
ENTRYPOINT ["/bin/ingest"]
