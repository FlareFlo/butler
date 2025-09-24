FROM docker.io/rust:1.90 as builder

WORKDIR /usr/src/app

# Get lockfile
COPY Cargo.toml Cargo.lock ./

# Cache deps and target folder
RUN --mount=type=cache,target=/usr/src/app/target \
    cargo fetch

# Build
COPY ./src ./src
RUN cargo build --release

FROM docker.io/debian:bookworm-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/butler .

CMD ["./butler"]