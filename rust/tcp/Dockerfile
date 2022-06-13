# Code from Alexander Brand
# https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/

# Also from https://kerkour.com/rust-small-docker-image

# Dockerfile for creating statically-linked Rust 
# application using Docker's multi-stage build feature.
FROM rust:lastest AS build
WORKDIR /usr/src

# Download the target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Create dummy project and build the app's dependencies.
RUN USER=root cargo new --bin trust
WORKDIR /usr/src/trust
COPY trust/Cargo.toml trust/Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application
COPY trust/src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container
FROM scratch
COPY --from=build /usr/local/cargo/bin/trust .
USER 1000
CMD ["./trust"]
