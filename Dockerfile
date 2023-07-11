# Use an official Rust runtime as a parent image
FROM rust:1.70 as builder

WORKDIR /usr/src/myapp

# Copy over your Manifest files
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Dummy build to cache dependencies
RUN mkdir src/ && echo "fn main() {}" > src/main.rs && cargo build --release

# Now copy the src and build again
COPY ./src ./src
RUN cargo build --release

# Our second stage will use Debian slim for a smaller final image
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/myapp/target/release/greg /usr/local/bin/greg

# Set the startup command to run your binary
CMD ["/usr/local/bin/greg"]
