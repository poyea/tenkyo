#!/bin/bash
set -e

echo "🦀 Building WASM module..."
source "$HOME/.cargo/env"
wasm-pack build --release --target web --out-dir www/pkg

echo "📦 WASM build complete!"
echo "Bundle size:"
ls -sh www/pkg/tenkyo_bg.wasm

echo ""
echo "✅ Build successful!"
echo "To test locally, run: cd www && python3 -m http.server 8080"
