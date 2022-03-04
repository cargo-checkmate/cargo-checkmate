FROM rust:1-bullseye

RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev
RUN cargo install cargo-checkmate

COPY ./entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
