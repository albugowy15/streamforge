FROM rust:1.95.0-trixie AS builder
WORKDIR /app
COPY apps/api/Cargo.toml apps/api/Cargo.lock ./
COPY apps/api/src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian13:nonroot-amd64
COPY --from=builder /app/target/release/api /app/api
EXPOSE 5000
CMD ["/app/api"]
