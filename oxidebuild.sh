#!/bin/bash
set -e
ROOT="$(cd "$(dirname "$0")" && pwd)"

# auto-detect binary names from Cargo.toml
BIN_NAME=$(grep -m1 '^name *= *"' "$ROOT/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
GUI_BIN=$(grep -m1 '^name *= *"' "$ROOT/gui/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
TARGET="${CARGO_BUILD_TARGET:-$(rustc -vV | grep host | awk '{print $2}')}"
TARGET_DIR="$ROOT/target/$TARGET/release"

echo "[*] Building $BIN_NAME (target: $TARGET)..."
cargo build --release --target "$TARGET" -j2

echo "[*] Building $GUI_BIN..."
cargo build --release --manifest-path "$ROOT/gui/Cargo.toml" -j2

echo "[+] Done. Binaries:"
ls -lh "$TARGET_DIR/$BIN_NAME" "$ROOT/gui/target/release/$GUI_BIN" 2>/dev/null ||
ls -lh "$TARGET_DIR/$BIN_NAME" "$ROOT/gui/target/release/$GUI_BIN.exe" 2>/dev/null ||
echo "    (check target dirs)"
