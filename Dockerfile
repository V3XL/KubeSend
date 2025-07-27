# Build stage
FROM rust:1.88-slim-trixie AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install curl
RUN apt-get update && \
    apt-get install -y curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 8080

WORKDIR /app
COPY --from=builder /app/target/release/kubesend /app
CMD ["./kubesend"]