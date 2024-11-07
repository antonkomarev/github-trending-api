# Build stage
FROM rust:bookworm AS builder

WORKDIR /app/
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/
COPY --from=builder /app/target/release/github-trending-api /app/github-trending-api
CMD ["/app/github-trending-api"]
