# =====================================================================================================================
# Build stage
# =====================================================================================================================

FROM rust:1.96-bookworm AS builder

WORKDIR /app

# Install native dependencies required to build the application
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency manifests first
COPY Cargo.toml Cargo.lock ./

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# =====================================================================================================================
# Runtime stage
# =====================================================================================================================

FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install only runtime dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled application from the builder stage
COPY --from=builder /app/target/release/RusPI /app/RusPI

# Copy the ONNX model
COPY onnx/resnet18-v1-7.onnx /app/onnx/resnet18-v1-7.onnx

# Create an unprivileged user
RUN useradd \
        --system \
        --no-create-home \
        --shell /usr/sbin/nologin \
        appuser \
    && chown -R appuser:appuser /app

USER appuser

EXPOSE 3000

CMD ["./RusPI"]