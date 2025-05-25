# Use official Rust image based on Debian (Linux)
FROM rustlang/rust:nightly-slim

# Set the working directory inside the container
WORKDIR /app

# Install build dependencies (optional, good for external crates)
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    apt-get clean

# Copy your code into the container
COPY . .

# Build the application (cached if Cargo.lock and src/ haven't changed)
RUN cargo build --release

# Default command to run the binary
CMD ["./target/release/metrics_collector"]
