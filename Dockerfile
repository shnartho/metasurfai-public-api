FROM rust:1.68.2 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev
WORKDIR /app
COPY --from=builder /app/target/release/metasurfai-public-api .
EXPOSE 8080
CMD ["./metasurfai-public-api"]