# ---- Build stage ----
FROM rust:1.90 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# ---- Final stage ----
FROM debian:trixie-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* /usr/share/doc /usr/share/man /usr/share/locales

# Set up working directory
WORKDIR /app

COPY --from=builder /app/target/release/rustify-telegram /app/rustify-telegram
# Command
ENTRYPOINT ["/app/rustify-telegram"]
