FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /build

# Creates the cache plan
FROM chef AS planner
COPY . .

# So it doesn't try building everything
RUN rm Cargo.toml; touch Cargo.toml; echo '[workspace]\nmembers = ["server", "common"]' > Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

RUN cargo build --release --bin server

FROM debian:buster-slim
EXPOSE 1337

WORKDIR /app
COPY --from=builder /build/target/release/server /usr/local/bin

CMD [ "server", "--ip", "0.0.0.0" ]