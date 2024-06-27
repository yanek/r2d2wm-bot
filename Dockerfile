FROM rust:1.79-slim AS builder

RUN apt-get update && apt-get -y install libc6
COPY . .
RUN cargo install --path r2d2wm-bot

CMD ["r2d2wm-bot"]
