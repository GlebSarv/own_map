FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY src/ ./src/

RUN cargo build --release

FROM debian:latest

WORKDIR /app

RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*
RUN apt policy libc6

COPY --from=builder /app/target/release/exif_reader .

CMD ["./exif_reader"]