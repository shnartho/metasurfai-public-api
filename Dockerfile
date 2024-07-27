FROM rust:latest as builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/metasurfai-public-api .
CMD ["./metasurfai-public-api"]
