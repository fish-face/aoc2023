FROM rust:1.74-alpine

RUN apk add --no-cache musl-dev bash

ADD entrypoint.sh /entrypoint.sh

VOLUME /code
WORKDIR /code
ENTRYPOINT ["/entrypoint.sh"]
