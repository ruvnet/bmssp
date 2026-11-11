#!/usr/bin/env bash
# Build the wasm module from this crate and stage it into npm/ for packing.
set -euo pipefail
cd "$(dirname "$0")/.."
# A host-level -fuse-ld=mold flag breaks rust-lld for wasm32.
unset RUSTFLAGS
wasm-pack build --release --target web --out-dir pkg --out-name bmssp_rust
cp LICENSE README.md npm/
cp pkg/bmssp_rust.js pkg/bmssp_rust.d.ts pkg/bmssp_rust_bg.wasm npm/
echo "staged npm/ from $(git rev-parse --short HEAD 2>/dev/null || echo worktree)"
