#!/bin/sh

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen .\target\wasm32-unknown-unknown\release\brackets-web-test.wasm --out-dir .\wasm_help\staging --no-modules --no-typescript

serve wasm_help