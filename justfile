set shell := ["bash", "-eu", "-o", "pipefail", "-c"]
coverage_min := env_var_or_default("PLEASE_COVERAGE_MIN", "45")

default:
    @just --list

setup:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v rustup >/dev/null 2>&1; then
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    fi
    if [ -f "$HOME/.cargo/env" ]; then
      source "$HOME/.cargo/env"
    fi
    rustup default stable
    rustup component add rustfmt clippy llvm-tools-preview
    cargo install --locked cargo-nextest || true
    cargo install --locked cargo-watch || true
    cargo install --locked cargo-audit || true
    cargo install --locked cargo-llvm-cov || true
    cargo install --locked just || true

fmt:
    cargo fmt --all --check

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    #!/usr/bin/env bash
    set -euo pipefail
    if command -v cargo-nextest >/dev/null 2>&1; then
      cargo nextest run --workspace --all-features
    else
      cargo test --workspace --all-features
    fi

cov:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
      cargo install --locked cargo-llvm-cov
    fi
    cargo llvm-cov --workspace --all-features --summary-only --fail-under-lines {{coverage_min}}

run *args:
    cargo run -p please-cli -- {{args}}

watch:
    cargo watch -x "check --workspace" -x "test --workspace"

ci: fmt lint test cov

bench:
    cargo bench --workspace

clean:
    cargo clean
    rm -rf .please
