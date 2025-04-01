FROM rust:slim-bookworm AS base

RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && apt-get install -y \
    libssl3 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

FROM base AS builder

# Install dependencies
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && apt-get install -y \
    clang \
    gnupg \
    libclang1 \
    libssl-dev \
    lld \
    nodejs \
    npm \
    pkg-config \
    sccache \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Download Rust toolchain
RUN rustup default stable

WORKDIR /sauropod

COPY package.json package-lock.json ./
COPY packages/ ./packages/

# Install Node.js dependencies
RUN npm ci

# Build the UI
RUN npm run build

COPY . ./

# Grab the Rust dependencies
RUN cargo fetch --locked

# Build the release binary
RUN cargo build --locked --profile=optimized-release --package sauropod-server

FROM base

COPY --from=builder /sauropod/target/optimized-release/sauropod-server /usr/local/bin/sauropod-server

CMD ["/usr/local/bin/sauropod-server"]
