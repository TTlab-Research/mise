# Build Rust extension in isolated environment
FROM rust:latest

WORKDIR /extension

# Copy project files
COPY . .

# Build release binary
RUN cargo build --release

# Verify binary was created
# Package "zed-mise" becomes "libzed_mise.so" in Rust
RUN ls -lh target/release/libzed_mise.so && \
    echo "✅ Build complete. Binary at: target/release/libzed_mise.so"
