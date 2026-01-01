# -------- Build stage --------
FROM rust:slim AS builder

WORKDIR /usr/src/aura-bot

RUN apt-get update && \
    apt-get install -y musl-tools ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Nightly toolchain
RUN rustup target add --toolchain nightly x86_64-unknown-linux-musl

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src

# Build release binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Runtime stage --------
FROM scratch AS runtime

WORKDIR /app

COPY --from=builder /usr/src/aura-bot/target/x86_64-unknown-linux-musl/release/aura-bot /aura-bot

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

ENTRYPOINT ["/aura-bot"]