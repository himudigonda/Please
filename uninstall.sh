#!/usr/bin/env bash
set -euo pipefail

print_usage() {
  cat <<'EOF'
Usage:
  ./uninstall.sh [--yes] [--purge-state] [--purge-please]

Options:
  --yes            Skip confirmation prompts.
  --purge-state    Remove all ~/.broski cache/tx/stage directories.
  --purge-please   Remove legacy ~/.please directories (if present).
EOF
}

ASSUME_YES=false
PURGE_STATE=false
PURGE_PLEASE=false

for arg in "${@:-}"; do
  case "$arg" in
    --yes)
      ASSUME_YES=true
      ;;
    --purge-state)
      PURGE_STATE=true
      ;;
    --purge-please)
      PURGE_PLEASE=true
      ;;
    --help|-h)
      print_usage
      exit 0
      ;;
    *)
      echo "unknown option: $arg"
      print_usage
      exit 1
      ;;
  esac
done

prompt_yes_no() {
  local message="$1"
  local response
  if [ "$ASSUME_YES" = true ]; then
    return 0
  fi

  read -r -p "$message [y/N]: " response
  response="$(printf '%s' "$response" | tr '[:upper:]' '[:lower:]')"
  case "${response}" in
    y|yes)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

remove_file_if_exists() {
  local path="$1"
  if [ -e "$path" ]; then
    rm -f "$path"
    echo "removed: $path"
  fi
}

remove_dir_if_exists() {
  local path="$1"
  if [ -d "$path" ]; then
    rm -rf "$path"
    echo "removed: $path"
  fi
}

echo "This script will uninstall Broski binaries and optionally clean old state directories."
if ! prompt_yes_no "Continue"; then
  echo "Uninstall canceled."
  exit 1
fi

declare -a BINARY_DIRS=(
  "$HOME/.local/bin"
  "$HOME/.cargo/bin"
  "/usr/local/bin"
  "/opt/homebrew/bin"
)

for dir in "${BINARY_DIRS[@]}"; do
  if [ -d "$dir" ]; then
    remove_file_if_exists "$dir/broski"
    if [ "$PURGE_PLEASE" = true ]; then
      if [ -e "$dir/please" ]; then
        remove_file_if_exists "$dir/please"
      fi
    fi
  fi
done

if [ "$PURGE_STATE" = true ]; then
  if prompt_yes_no "Purge user state at ~/.broski"; then
    remove_dir_if_exists "$HOME/.broski"
  fi
fi

if prompt_yes_no "Purge user cache at ~/.cache/broski"; then
  remove_dir_if_exists "$HOME/.cache/broski"
fi

if [ "$PURGE_PLEASE" = true ]; then
  if prompt_yes_no "Purge legacy ~/.please"; then
    remove_dir_if_exists "$HOME/.please"
  fi
fi

if command -v cargo >/dev/null 2>&1; then
  if prompt_yes_no "Remove installed Rust toolchain binary cargo-installed Broski artifacts"; then
    cargo uninstall broski-cli 2>/dev/null || true
    echo "requested cargo uninstall complete (if installed as cargo binary)"
  fi
fi

echo "Uninstall complete."
