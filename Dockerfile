FROM rust:latest as builder

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/myapp/target/release/my_api /usr/local/bin/my_api

EXPOSE 80

CMD ["my_api"]
