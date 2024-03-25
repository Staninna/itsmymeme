FROM rust:latest as builder

RUN USER=root cargo new --bin itsmymeme
WORKDIR /itsmymeme

COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info

WORKDIR /usr/local/bin

COPY --from=builder /itsmymeme/target/release/itsmymeme ./

EXPOSE 8080

CMD ["./itsmymeme"]
