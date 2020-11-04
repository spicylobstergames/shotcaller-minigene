#!/bin/sh

set -ex

cargo build --release --no-default-features --features wasm,opengl --target wasm32-unknown-unknown && \
wasm-bindgen target/wasm32-unknown-unknown/release/shotcaller.wasm --out-dir output --no-modules --no-typescript && \
(cd output && python3 -m http.server) || \
echo build failed
