FROM rust:latest

# Install system dependencies needed for Rust, MongoDB, PhysX, and Bevy dependencies
RUN apt update && apt install -y \
    libudev-dev \
    cmake \
    g++ \
    clang \
    libasound2-dev \
    mold

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Install Cranelift (optional, only if enabled in .cargo/config.toml)
RUN rustup component add rustc-codegen-cranelift || true

WORKDIR /web
COPY . .

CMD ["cargo", "watch", "-x", "check -x run"]

