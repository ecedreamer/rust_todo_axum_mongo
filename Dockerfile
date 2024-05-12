# Use a Rust image based on Debian Bullseye which has a newer GLIBC version
FROM rust:slim-bullseye AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/rust_todo_axum_mongo .

EXPOSE 8080

CMD ["./rust_todo_axum_mongo"]
