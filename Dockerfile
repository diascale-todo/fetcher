FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

# Install OpenSSL runtime libraries (libssl3) and CA certificates
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/fetcher /usr/local/bin/fetcher

# For debugging
# CMD ["sleep 3600"]
# Set the entrypoint to the fetcher binary
CMD ["fetcher"]
