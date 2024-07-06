FROM lukemathwalker/cargo-chef:latest-rust-1.79 AS chef
WORKDIR /

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get -y install libc6
COPY --from=planner /recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin r2d2wm-bot

FROM debian:bookworm-slim AS runtime
WORKDIR /
COPY --from=builder /target/release/r2d2wm-bot /usr/local/bin
ENTRYPOINT ["/usr/local/bin/r2d2wm-bot"]
