# Use an official Rust runtime as a parent image
FROM rust:1.70 as builder

WORKDIR /usr/src/greg

# Copy over your Manifest files
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build and cache dependencies
RUN mkdir src/ && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy the actual source code
COPY ./src ./src
RUN touch -a -m ./src/main.rs

# Build the application
RUN cargo build --release

# Our second stage will use Debian slim for a smaller final image
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/greg/target/release/greg /usr/local/bin/greg

# Set the startup command to run your binary
CMD ["/usr/local/bin/greg"]
