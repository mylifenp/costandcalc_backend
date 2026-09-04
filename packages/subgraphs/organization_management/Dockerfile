# syntax=docker/dockerfile:1.7
ARG PROJECT_DIR=packages/subgraphs/organization_management

########################################################
# Builder
########################################################
FROM ghcr.io/astral-sh/uv:python3.14-bookworm-slim AS builder
ARG PROJECT_DIR

ENV UV_COMPILE_BYTECODE=1 \
    UV_LINK_MODE=copy \
    UV_PYTHON_DOWNLOADS=never

WORKDIR /app

# Install dependencies first (better layer caching)
COPY ${PROJECT_DIR}/pyproject.toml ${PROJECT_DIR}/uv.lock ./
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-dev --no-install-project

# Copy source (main.py + organization_management/ module + tests/) and finalize sync
COPY ${PROJECT_DIR}/ .
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-dev

########################################################
# Runtime
########################################################
FROM python:3.14-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system app \
 && useradd --system \
    --gid app \
    --create-home \
    --home-dir /app \
    app

WORKDIR /app
COPY --from=builder --chown=app:app /app /app

ENV PATH="/app/.venv/bin:$PATH" \
    PYTHONPATH="/app/src"

USER app
EXPOSE 8000

ENTRYPOINT ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8001"]
