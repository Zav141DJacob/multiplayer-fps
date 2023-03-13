FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Creates the cache plan
FROM chef AS planner
COPY . .

# So it doesn't try building everything
RUN rm Cargo.toml; touch Cargo.toml; echo '[workspace]\nmembers = ["server", "common"]' > Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

RUN cargo build --release --bin server

FROM alpine:3.17
EXPOSE 1337

WORKDIR /app
COPY --from=builder /app/target/release/server /usr/local/bin

CMD [ "server" ]