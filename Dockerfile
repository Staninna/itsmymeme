FROM rust:latest as builder

RUN USER=root cargo new --bin itsmymeme
WORKDIR /itsmymeme

COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

ENV RUST_LOG=info
ENV PORT=8080

WORKDIR /usr/local/bin

COPY --from=builder /itsmymeme/target/release/itsmymeme ./

EXPOSE $PORT

CMD ["./itsmymeme"]
