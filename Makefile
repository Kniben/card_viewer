
wasm-bindgen: install-wasm-bindgen cargo-build
	mkdir -p wasm_bindgen
	bin/wasm-bindgen --out-dir wasm_bindgen/ --no-modules target/wasm32-unknown-unknown/debug/card_viewer.wasm

install-wasm-bindgen:
	command -v bin/wasm-bindgen >/dev/null 2>&1 || {cargo install --root . --debug wasm-bindgen-cli}

cargo-build:
	cargo build --verbose
