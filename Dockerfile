FROM docker.io/rust:1.90 as builder

WORKDIR /usr/src/app

# Get project code
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

# Cache deps and target folder
RUN --mount=type=cache,target=/usr/src/app/target \
    cargo fetch

# Build
RUN cargo build --release

FROM docker.io/debian:bookworm-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/butler .

CMD ["./butler"]