FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY proto/ ./proto/

RUN apt-get update && apt install -y protobuf-compiler
RUN cargo install protobuf-codegen

COPY src/ ./src/

RUN protoc --rust_out=. --proto_path=proto exif_readers_grpc.proto
RUN cargo clean
RUN cargo build --release

FROM debian:latest

WORKDIR /app

ENV KAFKA.BOOTSTRAPSERVER = ${KAFKA_BOOTSTRAPSERVER}
ENV KAFKA.TOPICS          = ${KAFKA_TOPICS}
ENV KAFKA.TIMEOUT         = ${KAFKA_TIMEOUT}
ENV GRPCSERVER.SERVER     = ${GRPCSERVER+SERVER}
ENV GRPCSERVER.PORT       = ${GRPCSERVER_PORT}

RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/exif_reader .

CMD ["./exif_reader"]
