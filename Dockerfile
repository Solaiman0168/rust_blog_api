FROM rust:1.77 AS builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust_blog_api .
COPY .env .env

CMD ["./rust_blog_api"]

