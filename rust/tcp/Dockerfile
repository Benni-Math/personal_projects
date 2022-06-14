# Code from Alexander Brand
# https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/

# Also from https://kerkour.com/rust-small-docker-image

# Dockerfile for creating statically-linked Rust 
# application using Docker's multi-stage build feature.
FROM rust:latest AS build
WORKDIR /usr/src

# Download the target for static linking
RUN rustup target add aarch64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# METHOD 1:

# Create dummy project and build the app's dependencies.
# RUN USER=root cargo new --bin trust
# WORKDIR /usr/src/trust
# COPY trust/Cargo.toml ./
# COPY trust/Cargo.lock ./ 
# RUN cargo build --release

# Copy the source and build the application
# COPY trust/src ./src
# RUN cargo install --target aarch64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container
# FROM scratch
# COPY --from=build /usr/local/cargo/bin/trust .
# USER 1000
# CMD ["./trust"]



# METHOD 2

COPY ./trust .
RUN cargo build --target aarch64-unknown-linux-musl --release

FROM scratch

WORKDIR /trust

COPY --from=build
/usr/src/trust/target/aarch64-unknown-linux-musl/release/trust ./

USER 1000
CMD ["/trust/trust"]

