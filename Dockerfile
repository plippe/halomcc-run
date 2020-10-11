FROM rust:1.46-alpine3.12 as server

RUN apk add --no-cache musl-dev openssl-dev

COPY . /opt/repository
WORKDIR /opt/repository

RUN cargo build --release

# ---

FROM alpine:3.12

COPY --from=server /opt/repository/target/release/halomcc-run /opt/bin/server
COPY ./resources /opt/bin/resources
WORKDIR /opt/bin

ENTRYPOINT PORT=${PORT} /opt/bin/server
HEALTHCHECK --interval=1s --timeout=1s --retries=30 \
  CMD curl -f http://0.0.0.0:${PORT} || exit 1
