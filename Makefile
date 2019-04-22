
wasm-bindgen: cargo-build
	mkdir -p wasm_bindgen
	bin/wasm-bindgen --out-dir wasm_bindgen/ --no-modules target/wasm32-unknown-unknown/debug/card_viewer.wasm

cargo-build:
	cargo build --verbose
