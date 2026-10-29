FROM rust:1.77 as builder
WORKDIR /app
COPY Cargo.toml .
COPY src src
COPY migrations migrations
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/funeral-os-api /usr/local/bin/api
CMD ["api"]
