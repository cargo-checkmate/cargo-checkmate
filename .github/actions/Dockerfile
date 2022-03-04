FROM debian:stable

RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-installer.sh
RUN sh ./rustup-installer.sh -y
RUN sh -c '. $HOME/.cargo/env && cargo install cargo-checkmate'

COPY ./entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
