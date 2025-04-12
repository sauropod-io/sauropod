FROM rust:slim-bookworm AS base

RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && apt-get install -y \
    libssl3 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

FROM node:23-bookworm-slim AS npm-builder

WORKDIR /sauropod

# NPM MARKER
COPY package-lock.json package-lock.json
COPY package.json package.json
COPY packages/client/package.json packages/client/package.json
COPY packages/ui/package.json packages/ui/package.json

# Install Node.js dependencies
RUN npm ci

COPY packages/ ./packages/

# Build the UI
RUN npm run build

FROM base AS rust-builder

# Install dependencies
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && apt-get install -y \
    gnupg \
    libclang1 \
    libssl-dev \
    nodejs \
    npm \
    pkg-config \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /sauropod

# Download Rust toolchain
RUN rustup default stable

# CARGO MARKER
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY crates/config/Cargo.toml crates/config/Cargo.toml
COPY crates/core-tools/Cargo.toml crates/core-tools/Cargo.toml
COPY crates/database/Cargo.toml crates/database/Cargo.toml
COPY crates/http/Cargo.toml crates/http/Cargo.toml
COPY crates/json-schema/Cargo.toml crates/json-schema/Cargo.toml
COPY crates/llm-inference/Cargo.toml crates/llm-inference/Cargo.toml
COPY crates/logging/Cargo.toml crates/logging/Cargo.toml
COPY crates/mcp/Cargo.toml crates/mcp/Cargo.toml
COPY crates/prompt-templates/Cargo.toml crates/prompt-templates/Cargo.toml
COPY crates/schemas/Cargo.toml crates/schemas/Cargo.toml
COPY crates/server/Cargo.toml crates/server/Cargo.toml
COPY crates/task-context/Cargo.toml crates/task-context/Cargo.toml
COPY crates/task/Cargo.toml crates/task/Cargo.toml
COPY crates/tool-spec/Cargo.toml crates/tool-spec/Cargo.toml
COPY crates/workflows/Cargo.toml crates/workflows/Cargo.toml
COPY tools/create-release-tag/Cargo.toml tools/create-release-tag/Cargo.toml
COPY tools/generate-code/Cargo.toml tools/generate-code/Cargo.toml
RUN mkdir crates/config/src && touch crates/config/src/lib.rs && \
    mkdir crates/core-tools/src && touch crates/core-tools/src/lib.rs && \
    mkdir crates/database/src && touch crates/database/src/lib.rs && \
    mkdir crates/http/src && touch crates/http/src/lib.rs && \
    mkdir crates/json-schema/src && touch crates/json-schema/src/lib.rs && \
    mkdir crates/llm-inference/src && touch crates/llm-inference/src/lib.rs && \
    mkdir crates/logging/src && touch crates/logging/src/lib.rs && \
    mkdir crates/mcp/src && touch crates/mcp/src/lib.rs && \
    mkdir crates/prompt-templates/src && touch crates/prompt-templates/src/lib.rs && \
    mkdir crates/schemas/src && touch crates/schemas/src/lib.rs && \
    mkdir crates/server/src && touch crates/server/src/lib.rs && \
    mkdir crates/task-context/src && touch crates/task-context/src/lib.rs && \
    mkdir crates/task/src && touch crates/task/src/lib.rs && \
    mkdir crates/tool-spec/src && touch crates/tool-spec/src/lib.rs && \
    mkdir crates/workflows/src && touch crates/workflows/src/lib.rs && \
    mkdir tools/create-release-tag/src && touch tools/create-release-tag/src/lib.rs && \
    mkdir tools/generate-code/src && touch tools/generate-code/src/lib.rs

# Grab the Rust dependencies
RUN cargo fetch --locked

COPY --from=npm-builder /sauropod/packages/ui/dist /sauropod/packages/ui/dist
COPY . ./

# Build the release binary
RUN cargo build --locked --profile=optimized-release --package sauropod-server

FROM base

COPY --from=rust-builder /sauropod/target/optimized-release/sauropod-server /usr/local/bin/sauropod-server

CMD ["/usr/local/bin/sauropod-server"]
