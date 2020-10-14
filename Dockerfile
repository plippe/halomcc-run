FROM ekidd/rust-musl-builder:1.47.0 AS builder

COPY --chown=rust:rust . /opt/repository
WORKDIR /opt/repository

RUN cargo build --release

# ---

FROM alpine:3.12

COPY --from=builder /opt/repository/target/x86_64-unknown-linux-musl/release/halomcc-run /opt/bin/server
WORKDIR /opt/bin

ENTRYPOINT PORT=${PORT} /opt/bin/server
HEALTHCHECK --interval=1s --timeout=1s --retries=30 \
  CMD curl -f http://0.0.0.0:${PORT} || exit 1
