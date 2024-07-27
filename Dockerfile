FROM rust:latest as builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo fetch
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/metasurfai-public-api .
CMD ["./metasurfai-public-api"]
