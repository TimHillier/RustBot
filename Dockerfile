FROM rust:latest
LABEL authors="Tim"

WORKDIR /app
COPY ./ ./
ENV database=sqlite:rustbot.sqlite

RUN cargo install sqlx-cli
RUN sqlx database create --database-url $database
RUN sqlx migrate run --database-url $database
RUN cargo sqlx prepare --database-url $database
RUN cargo build --release

CMD ["./target/release/RustBot"]
