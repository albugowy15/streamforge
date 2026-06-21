FROM rust:1.95.0-trixie AS builder
WORKDIR /app
COPY apps/api/Cargo.toml apps/api/Cargo.lock ./
COPY apps/api/src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian13:nonroot-amd64
WORKDIR /app
COPY --from=builder /app/target/release/streamforge_api /app/streamforge_api
COPY apps/api/migrations /app/migrations
EXPOSE 5000
CMD ["/app/streamforge_api"]
