FROM rust:1.23-jessie as build

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./identicons/ ./identicons/
COPY ./identicons-server/ ./identicons-server/

RUN cargo build --release

# ----------

# Create a new stage with a minimal image
FROM debian:jessie-slim as production
WORKDIR /app

# Copies the binary from the "build" stage to the current stage
COPY --from=build /app/target/release/identicons_server .
COPY --from=build /app/identicons-server/templates ./templates

ENV PORT=8080 \
    HOST=0.0.0.0
EXPOSE $PORT
CMD ["/app/identicons_server"]
