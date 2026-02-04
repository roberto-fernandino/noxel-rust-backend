# syntax=docker/dockerfile:1

FROM rust:1.76 as builder
WORKDIR /app

# Cache deps
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
RUN cargo build --release

# Build real app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/noxel-rust-backend /app/noxel-rust-backend
EXPOSE 8080
CMD ["/app/noxel-rust-backend"]
