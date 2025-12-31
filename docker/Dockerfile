# -------- Build stage --------
FROM rust:1.75 AS builder

WORKDIR /app
COPY . /app
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Runtime stage --------
FROM scratch

COPY --from=builder \
    /app/target/x86_64-unknown-linux-musl/release/aura-bot \
    /aura-bot

CMD ["/aura-bot"]