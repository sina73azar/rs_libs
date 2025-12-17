#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

COUNTER_FILE="$PROJECT_ROOT/.build_counter"

if [[ -f "$COUNTER_FILE" ]]; then
  COUNTER=$(cat "$COUNTER_FILE")
else
  COUNTER=0
fi

COUNTER=$((COUNTER + 1))
echo "$COUNTER" > "$COUNTER_FILE"


COPIED_BIN_NAME="rust_steps_$COUNTER"
BIN_NAME="rust_steps"
OUT_DIR="/home/azarfarshi.s@drp.local/Documents/rs_bin"

echo "Building release binary..."
cd "$PROJECT_ROOT"
cargo build --release

echo "Deploying binary to $OUT_DIR..."
mkdir -p "$OUT_DIR"

cp -f "target/release/$BIN_NAME" "$OUT_DIR/$COPIED_BIN_NAME"
chmod +x "$OUT_DIR/$COPIED_BIN_NAME"

echo "Done. Binary available at:"
echo "$OUT_DIR/$COPIED_BIN_NAME"
"$COPIED_BIN_NAME"
