#!/bin/bash
set -e

echo "[*] Building all targets..."
cargo build --release -j 2

echo "[*] Copying to root..."
cp target/release/oxide ./oxide
cp target/release/liboxide_proxy.so ./liboxide_proxy.so 2>/dev/null || true
cp target/release/libhypersecurity.so ./libhypersecurity.so 2>/dev/null || true

echo "[+] Done:"
ls -lh oxide liboxide_proxy.so libhypersecurity.so 2>/dev/null
