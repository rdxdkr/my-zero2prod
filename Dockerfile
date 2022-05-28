# We use the latest Rust stable release as base image
# Builder stage
FROM rust:1.59.0 AS builder

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .

# Force sqlx to look at the saved metadata instead of
# trying to query a live db
ENV SQLX_OFFLINE true

# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release

# Runtime stage
FROM rust:1.59.0 AS runtime

WORKDIR /app

# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/my-zero2prod my-zero2prod

# We need the configuration file at runtime!
COPY configuration configuration

# Use the production environment to accept connections from
# any network interface
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./my-zero2prod"]