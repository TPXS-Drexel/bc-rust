FROM rust:latest

WORKDIR /usr/src/bc-rust

COPY . .

RUN cargo build --release

CMD cargo run