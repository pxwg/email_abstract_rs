# rust version: 1.84
FROM rust:1.84-slim as builder

WORKDIR /app
COPY . .

# Install SQLite and other dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the application in release mode
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Replace "email_abstract_rs" with your actual binary name from Cargo.toml
COPY --from=builder /app/target/release/email_abstract_rs /app/email_abstract_rs

ENTRYPOINT ["/app/email_abstract_rs"]
