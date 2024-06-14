FROM rust:latest as builder

# Install necessary system dependencies
RUN apt-get update && apt-get install -y libpq-dev

# Create a new user and switch to it
RUN useradd -ms /bin/bash appuser
USER appuser
WORKDIR /home/appuser

# Copy the project files
COPY --chown=appuser:appuser . .

# Install Diesel CLI as the new user
RUN cargo install diesel_cli --no-default-features --features postgres

# Build the project
RUN cargo build --release

FROM debian:buster-slim

# Install necessary system dependencies
RUN apt-get update && apt-get install -y libpq-dev

# Copy the built binary from the builder stage
COPY --from=builder /home/appuser/target/release/matrix_backend /usr/local/bin/

CMD ["matrix_backend"]
