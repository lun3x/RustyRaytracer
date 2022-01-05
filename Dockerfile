FROM rust:1.57 AS chef
RUN cargo install cargo-chef
WORKDIR /raytracer

FROM chef AS planner
COPY src benches Cargo.toml Cargo.lock ./
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /raytracer/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY src benches Cargo.toml Cargo.lock ./
RUN cargo build --release

# the final base
FROM debian:buster-slim AS runtime
WORKDIR /raytracer
# copy the build artifact from the build stage
COPY --from=builder /raytracer/target/release/raytracer /usr/local/bin

# set the startup command to run the binary
ENTRYPOINT ["/usr/local/bin/raytracer"]