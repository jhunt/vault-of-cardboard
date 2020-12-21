FROM ubuntu:18.04

LABEL maintainer="images@huntprod.com"

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y squid \
 && rm -rf /var/lib/apt/lists/*

COPY oci/proxy/entrypoint.sh /sbin/entrypoint.sh
EXPOSE 3128/tcp
ENTRYPOINT ["/sbin/entrypoint.sh"]
