# https://hub.docker.com/_/rust/
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /usr/src/benchmark
COPY . .

RUN cargo install --path benchmark

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/benchmark /usr/local/bin/benchmark
COPY ./benchmark/compare_resource_latency.sh .
COPY ./benchmark/compare_kind_latency.sh .

# No entry point, since I want to use the shell files as well.
CMD [ "benchmark" ]
