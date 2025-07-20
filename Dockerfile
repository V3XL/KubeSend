# Build stage
FROM rust:1.88-slim-trixie AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

EXPOSE 8080

WORKDIR /app
COPY --from=builder /app/target/release/kubesend /app
CMD ["./kubesend"]