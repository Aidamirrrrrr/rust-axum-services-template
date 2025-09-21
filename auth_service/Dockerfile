FROM rust:1.90.0 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY ./migrations ./migrations

COPY src ./src

RUN cargo fetch --locked

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/auth_service /app/auth_service
RUN chmod +x /app/auth_service
EXPOSE 8080
ENTRYPOINT ["/app/auth_service"]
