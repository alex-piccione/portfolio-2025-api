# Build stage - use specific version with updates
FROM rust:1.89-bookworm AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/api_axum*

# Copy real source and build
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build the application
RUN cargo build --release


# Runtime stage - use distroless for minimal attack surface
#FROM debian:bookworm-slim
FROM gcr.io/distroless/cc-debian12

# Copy the binary
COPY --from=builder /app/target/release/api_axum /usr/local/bin/api_axum

# receive a value as ARG and pass it to the container as ENV
ARG CONFIGURATION_FILE
ENV CONFIGURATION_FILE=$CONFIGURATION_FILE

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/api_axum"]
