FROM rust:latest
LABEL authors="Tim"

WORKDIR /app
COPY ./ ./
ENV database=sqlite:data/rustbot.sqlite

RUN cargo install sqlx-cli
RUN cargo sqlx database create --database-url $database
RUN cargo sqlx migrate info --database-url sqlite:data/rustbot.sqlite --source data/migrations
RUN cargo sqlx migrate run --database-url $database --source data/migrations
RUN cargo sqlx migrate info --database-url sqlite:data/rustbot.sqlite --source data/migrations
RUN cargo sqlx prepare --database-url $database
RUN cargo build --release


VOLUME /data
CMD ["./target/release/RustBot"]