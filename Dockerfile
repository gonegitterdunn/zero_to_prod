#### CHEF STAGE
FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 as chef
# Same as 'cd app'. The 'app' folder will be created if it doesn't already exist
WORKDIR /app
# Install required system deps for linking configuration
RUN apt update && apt install lld clang -y

### PLANNER STAGE -- COMPUTES RECIPE FILE
FROM chef AS planner
# Copy all files from the working environment to the Docker image
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

### BUILDER STAGE -- CACHES DEPENDENCIES AND BUILDS BINARY
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
# Allows sqlx to check queries at compile time without needing a db connection
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero_to_prod

# RUNTIME STAGE
FROM debian:bullseye-slim@sha256:f52f9aebdd310d504e0995601346735bb14da077c5d014e9f14017dadc915fe5 AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment to our runtime environment
COPY --from=builder /app/target/release/zero_to_prod zero_to_prod
# Need the config file at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production
# When 'docker run' is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]