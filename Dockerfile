
# Use the Rust base image
FROM rust:latest

# Install the protobuf-compiler (protoc)
RUN apt-get update && apt-get install -y protobuf-compiler

# Set the working directory for the app
WORKDIR /app

# Copy all necessary files except those specified in .dockerignore
COPY ../ .

# Ensure that the correct working directory is set before building
WORKDIR /app/minioc
RUN cargo build --release

WORKDIR /app/messengerc
RUN cargo build --release

# Set the entry point (adjust as necessary)
# CMD ["cargo", "run", "--release", "--bin", "messenger"]

