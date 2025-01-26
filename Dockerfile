FROM rust:1.84-bookworm AS builder

RUN apt update && apt upgrade -y 
RUN apt install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross gcc-arm-linux-gnueabihf

RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64

WORKDIR /build
COPY src/ /build/src/
COPY Cargo.toml /build/

RUN cargo build --target aarch64-unknown-linux-gnu --release


FROM rust:1.84-slim-bookworm
WORKDIR /app
COPY --from=builder /build/target/aarch64-unknown-linux-gnu/release/weight-tracker /app/
COPY templates/ /app/templates/
COPY static /app/static/


RUN chmod +x weight-tracker
EXPOSE 8000