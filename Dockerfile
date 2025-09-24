FROM docker.io/rust:1.90 as builder

WORKDIR /usr/src/app

# Get project code
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

# Re-use locally mounted target folder
# Build
RUN --mount=type=bind,source=./target,target=/usr/src/app/target \
    cargo build --release

FROM docker.io/debian:bookworm-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/butler .

CMD ["./butler"]