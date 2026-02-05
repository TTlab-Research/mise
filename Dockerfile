# Build Rust extension in isolated environment
FROM rust:latest

WORKDIR /extension

# Copy project files
COPY . .

# Build release binary
RUN cargo build --release

# Binary will be at: target/release/libmise.dylib (macOS)
#                    target/release/libmise.so (Linux)
#                    target/release/mise.dll (Windows)
