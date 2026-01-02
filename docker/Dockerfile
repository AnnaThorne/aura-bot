# -------- Build stage --------
FROM rust:slim AS builder

WORKDIR /usr/src/aura-bot

# Install musl toolchain, CA certs and common build deps for native crates
RUN apt-get update && \
    apt-get install -y musl-tools ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Ensure the nightly toolchain and add the musl target for nightly explicitly
RUN rustup toolchain install nightly && \
    rustup target add x86_64-unknown-linux-musl --toolchain nightly

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create empty src to allow dependency build caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo +nightly build --release --target x86_64-unknown-linux-musl

# Copy source
COPY src ./src

# Build release binary
RUN cargo +nightly build --release --target x86_64-unknown-linux-musl

# -------- Runtime stage --------
FROM scratch AS runtime

WORKDIR /app

COPY --from=builder /usr/src/aura-bot/target/x86_64-unknown-linux-musl/release/aura-bot /aura-bot

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

ENTRYPOINT ["/aura-bot"]