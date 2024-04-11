# Builder stage

# We use the latest Rust stable release as base image
FROM rust:1.77.2 AS builder
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release


# Runtime stage
FROM rust:1.77.2-slim AS runtime

WORKDIR /app
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/midnight_library midnight_library

# We need the configuration file at runtime!
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/midnight_library"]