FROM rust:1.97.1-alpine AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

COPY Cargo.toml ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN cargo build --release

FROM alpine:3.20 AS runner
WORKDIR /app
RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/cart_service ./cart_service

EXPOSE 3001
CMD ["./cart_service"]