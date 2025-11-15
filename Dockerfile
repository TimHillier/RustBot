# Build stage
FROM rust:latest AS builder
LABEL authors="Tim"

WORKDIR /app
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./data ./data

ENV database=sqlite:data/rustbot.sqlite

RUN cargo install sqlx-cli
RUN cargo sqlx database create --database-url $database
RUN cargo sqlx migrate info --database-url sqlite:data/rustbot.sqlite --source data/migrations
RUN cargo sqlx migrate run --database-url $database --source data/migrations
RUN cargo sqlx migrate info --database-url sqlite:data/rustbot.sqlite --source data/migrations
RUN cargo sqlx prepare --database-url $database
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (SQLite for sqlx)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the compiled binary from the builder stage
COPY --from=builder /app/target/release/RustBot /app/RustBot
COPY --from=builder /app/data/migrations /app/data/migrations

# Create data directory for SQLite database
RUN mkdir -p /data

VOLUME /data

CMD ["./RustBot"]