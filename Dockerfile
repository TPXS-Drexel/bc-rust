FROM rust:latest

WORKDIR /usr/src/bc-rust

COPY . .

EXPOSE 8080

RUN cargo build --release

CMD cargo run