# Stage 1: Build Frontend
FROM node:26-alpine AS frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ .
RUN npm run build

# Stage 2: Build Backend
FROM debian:bookworm-slim AS backend-builder
WORKDIR /app
# Install build dependencies and rustup
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl build-essential&& rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# Install nightly toolchain
RUN rustup toolchain install nightly
# Copy manifests
COPY Cargo.toml Cargo.lock ./
# Pre-build dependencies (cache layer) using nightly
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo +nightly build --release && rm -rf src
# Copy actual source
COPY . .
RUN cargo +nightly build --release

# Stage 3: Final Runtime
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=backend-builder /app/target/release/jellynizer .

# Copy frontend assets from frontend-builder
# Note: The backend expects assets in ../frontend/dist relative to the binary
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

# Set environment
ENV RUST_LOG=info
EXPOSE 3000

# Run the application
CMD ["./jellynizer"]