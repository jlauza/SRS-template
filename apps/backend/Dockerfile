# ---- Build Stage ----
FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

# Install build dependencies for linking with PostgreSQL
RUN apt-get update && apt-get install -y libpq-dev pkg-config

# Build the release binary with nightly toolchain (for edition2024)
RUN cargo build --release

# ---- Runtime Stage ----#
FROM rust:1.77

WORKDIR /app

# Install runtime dependencies for PostgreSQL
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Copy any necessary runtime files (e.g., .env)
# COPY --from=builder /app/.env /app/.env

EXPOSE 8080

CMD ["/app/backend"]