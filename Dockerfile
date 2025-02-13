
# Use the Rust official image as a base
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo files and download dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Expose the Rocket default port
EXPOSE 8000

# Run the application
CMD ["./target/release/your_api_binary"]
