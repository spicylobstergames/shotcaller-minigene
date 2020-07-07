#!/bin/sh

cargo build --release --target wasm32-unknown-unknown && \
wasm-bindgen target/wasm32-unknown-unknown/release/shotcaller.wasm --out-dir output --no-modules --no-typescript && \
(cd output && python3 -m http.server) || \
echo build failed
