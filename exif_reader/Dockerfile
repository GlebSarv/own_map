FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY src/ ./src/

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN sudo apt update && apt-get install -y extra-runtime-dependencies &&  apt install libc6 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/exif_reader .

CMD ["./exif_reader"]