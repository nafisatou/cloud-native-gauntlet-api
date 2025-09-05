# -------------------------
# Stage 1: Builder
# -------------------------
FROM rust:1.87-bookworm as builder

# Set working directory
WORKDIR /usr/src/app

# Copy manifest files first for caching dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy src to allow cargo to fetch dependencies (speeds up rebuilds)
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies
RUN cargo fetch

# Remove dummy src
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the Rust API in release mode
RUN cargo build --release

# -------------------------
# Stage 2: Runtime
# -------------------------
FROM debian:bookworm-slim

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-api .

# Expose port
EXPOSE 8081

# Run the Rust API
CMD ["./rust-api"]
