FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

CMD ["./target/release/simple-api-with-rust"]
