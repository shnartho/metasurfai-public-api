FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates

WORKDIR /app
COPY --from=builder /app/target/release/metasurfai-public-api .
EXPOSE 8080
CMD ["./metasurfai-public-api"]