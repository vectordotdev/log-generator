# Use Rust as the base image
FROM rust:1.85-slim-bookworm

# Set the working directory inside the container
WORKDIR /usr/src/log_generator

# Copy the necessary files into the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Set the default command to run the log generator with the desired arguments
CMD ["./target/release/log-generator"]
