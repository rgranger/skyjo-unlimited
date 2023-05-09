FROM rust:1.69-alpine

WORKDIR /app

# Install musl-tools to make many crates compile successfully
RUN apk add --no-cache musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

VOLUME ["/usr/local/cargo"]
