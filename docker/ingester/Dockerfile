FROM alpine:3

RUN apk update \
 && apk add curl ca-certificates jq \
 && rm -rf /var/cache/apk/* \
 && curl -Lo /usr/bin/s3 https://github.com/jhunt/s3/releases/download/v0.2.7/s3-linux-amd64 \
 && chmod 0755 /usr/bin/s3

ADD ingest /bin/ingest

WORKDIR /workspace
ENTRYPOINT ["/bin/ingest"]
