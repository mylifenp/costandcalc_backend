# syntax=docker/dockerfile:1.7

########################################################
# Builder
########################################################
FROM node:24-bookworm AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    pkg-config \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Install Rust
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
ENV PATH="${CARGO_HOME}/bin:${PATH}"

RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal

WORKDIR /workspace

# Install Node dependencies first (better layer caching)
COPY package*.json ./

RUN --mount=type=cache,target=/root/.npm \
    npm ci

# Copy workspace
COPY . .

# Build
RUN --mount=type=cache,target=/opt/cargo/registry \
    --mount=type=cache,target=/opt/cargo/git \
    --mount=type=cache,target=/workspace/target \
    ./node_modules/.bin/nx run subgraphs_work:build:production

########################################################
# Runtime
########################################################
FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Create an unprivileged user
RUN groupadd --system app \
 && useradd --system \
    --gid app \
    --create-home \
    --home-dir /app \
    app

WORKDIR /app

COPY --from=builder \
    /workspace/dist/target/subgraphs_work/release/subgraphs_work \
    /app/subgraphs_work

RUN chown app:app /app/subgraphs_work \
    && chmod 755 /app/subgraphs_work

USER app

EXPOSE 8001

ENTRYPOINT ["/app/subgraphs_work"]
