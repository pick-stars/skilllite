#!/usr/bin/env bash
# Dev-only: install skilllite to ~/.skilllite/bin when missing or forced.
# Skips bundling into src-tauri/resources (production prebuild does that).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ASSISTANT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ROOT="$(cd "$ASSISTANT_DIR/../.." && pwd)"
BIN_DIR="${HOME}/.skilllite/bin"
SKILLLITE_BIN="${BIN_DIR}/skilllite"

if [[ -x "${SKILLLITE_BIN}" && "${SKILLLITE_FORCE_PREBUILD:-}" != "1" ]]; then
  echo "dev prebuild: using ${SKILLLITE_BIN} (set SKILLLITE_FORCE_PREBUILD=1 to reinstall)"
  exit 0
fi

cd "$ROOT"
mkdir -p "${BIN_DIR}"
rm -f "${SKILLLITE_BIN}"
cargo install --path skilllite --features memory_vector --root "${HOME}/.skilllite" --force
echo "dev prebuild: installed ${SKILLLITE_BIN}"
