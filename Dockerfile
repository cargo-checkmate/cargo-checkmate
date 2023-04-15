FROM rust:1-bookworm

RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev
RUN rustup component add clippy rustfmt
RUN cargo install cargo-checkmate

ENTRYPOINT ["cargo-checkmate"]
