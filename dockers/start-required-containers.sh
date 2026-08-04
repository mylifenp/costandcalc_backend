#!/usr/bin/env bash
set -euo pipefail

# Run from this script's directory so docker-compose.yml is always discovered.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

docker compose up -d mongodb redis redis-ps minio minio-buckets
