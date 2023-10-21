FROM rust:latest
LABEL authors="Tim"

WORKDIR /app
COPY /migrations /app
COPY /src /app

RUN cargo build --release

CMD["./target/release/RustBot"]
