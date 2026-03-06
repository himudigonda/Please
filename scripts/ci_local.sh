#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "[ci_local] single-runner validation start"
echo "[ci_local] 1/4 fmt"
cargo fmt --all --check

echo "[ci_local] 2/4 clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "[ci_local] 3/4 tests"
cargo test --workspace

echo "[ci_local] 4/4 docs build"
cd website
npm ci
npm run build

echo "[ci_local] all checks passed"
