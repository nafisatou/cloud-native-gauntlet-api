# ---- Runtime stage only ----
FROM scratch
WORKDIR /app

# Copy the prebuilt dummy binary (make sure it's executable on host)
COPY rust-api /app/rust-api

# Run the dummy Rust API
CMD ["/app/rust-api"]
