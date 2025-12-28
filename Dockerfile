FROM rust:1.90 AS builder

WORKDIR /app
COPY . .

# Determine target based on platform
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        rustup target add x86_64-unknown-linux-musl && \
        apt-get update && apt-get install -y musl-tools && \
        rm -rf /var/lib/apt/lists/* && \
        cargo build --target x86_64-unknown-linux-musl --release && \
        strip target/x86_64-unknown-linux-musl/release/rustify-telegram; \
    else \
        rustup target add aarch64-unknown-linux-musl && \
        apt-get update && apt-get install -y musl-tools && \
        rm -rf /var/lib/apt/lists/* && \
        cargo build --target aarch64-unknown-linux-musl --release && \
        strip target/aarch64-unknown-linux-musl/release/rustify-telegram; \
    fi

FROM scratch

ARG TARGETARCH
# Copy the built binary
COPY --from=builder /app/target/*-unknown-linux-musl/release/rustify-telegram /app/rustify-telegram

ENTRYPOINT ["/app/rustify-telegram"]