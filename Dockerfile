FROM rust:1.23-jessie as build

# Creates a dummy project used to build dependencies
# This helps caching
RUN USER=root cargo new --bin app
WORKDIR /app

# Copies over *only* dependency manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies
RUN cargo build --release
# Remove the fake source code
RUN rm ./src/*.rs
# Remove the fake binary
RUN rm target/release/identicons

# Copies the actual source code
COPY ./src ./src
COPY ./templates ./templates

# Build the app
RUN cargo build --release

# ----------

# Create a new stage with a minimal image
FROM debian:jessie-slim as production
WORKDIR /app

# Copies the binary from the "build" stage to the current stage
COPY --from=build /app/target/release/identicons .
COPY --from=build /app/templates ./templates

ENV PORT=8080 \
    HOST=0.0.0.0
EXPOSE $PORT
CMD ["/app/identicons"]
