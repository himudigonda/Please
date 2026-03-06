#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_FILE="${ROOT_DIR}/artifacts/prove-cache.log"
STYLE_FILE="${ROOT_DIR}/frontend/src/styles.css"
BACKEND_FILE="${ROOT_DIR}/backend/src/main.rs"
STYLE_BACKUP="$(mktemp)"
BACKEND_BACKUP="$(mktemp)"

resolve_broski_bin() {
  if [ -n "${BROSKI_BIN:-}" ] && [ -x "${BROSKI_BIN:-}" ]; then
    printf '%s\n' "${BROSKI_BIN}"
    return
  fi

  local candidates=(
    "${ROOT_DIR}/../../target/debug/broski"
    "${ROOT_DIR}/../../../../../target/debug/broski"
  )

  local candidate
  for candidate in "${candidates[@]}"; do
    if [ -x "$candidate" ]; then
      printf '%s\n' "$candidate"
      return
    fi
  done

  if command -v broski >/dev/null 2>&1; then
    command -v broski
    return
  fi

  echo "unable to resolve Broski binary; set BROSKI_BIN explicitly" >&2
  exit 1
}

BROSKI_BIN="$(resolve_broski_bin)"

mkdir -p "${ROOT_DIR}/artifacts"
: > "$LOG_FILE"
cp "$STYLE_FILE" "$STYLE_BACKUP"
cp "$BACKEND_FILE" "$BACKEND_BACKUP"
cleanup() {
  cp "$STYLE_BACKUP" "$STYLE_FILE"
  cp "$BACKEND_BACKUP" "$BACKEND_FILE"
  rm -f "$STYLE_BACKUP" "$BACKEND_BACKUP"
}
trap cleanup EXIT

run_and_log() {
  local label="$1"
  shift
  echo "=== ${label} ===" | tee -a "$LOG_FILE"
  (cd "$ROOT_DIR" && "$BROSKI_BIN" --workspace . "$@") 2>&1 | tee -a "$LOG_FILE"
  echo | tee -a "$LOG_FILE"
}

run_and_log "cold build" run package_container --explain
run_and_log "warm build" run package_container --explain

printf '\n' >> "$STYLE_FILE"
echo '/* prove-cache frontend mutation */' >> "$STYLE_FILE"
run_and_log "frontend mutation" run package_container --explain

printf '\n' >> "$BACKEND_FILE"
echo '// prove-cache backend mutation' >> "$BACKEND_FILE"
run_and_log "backend mutation" run package_container --explain

echo "proof log written to ${LOG_FILE}"
