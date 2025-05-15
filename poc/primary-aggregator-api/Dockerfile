# https://hub.docker.com/_/rust/
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /usr/src/api_server
COPY . .

RUN cargo install --path primary-aggregator-api

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/primary-aggregator-api /usr/local/bin/primary-aggregator-api

ENTRYPOINT ["primary-aggregator-api"]
