
wasm-bindgen: cargo-build
	mkdir -p wasm_bindgen
	wasm-bindgen --out-dir wasm_bindgen/ --no-modules target/wasm32-unknown-unknown/debug/card_viewer.wasm

wasm-bindgen-release: cargo-build-release
	mkdir -p wasm_bindgen
	wasm-bindgen --out-dir wasm_bindgen/ --no-modules target/wasm32-unknown-unknown/release/card_viewer.wasm

cargo-build:
	cargo build --verbose

cargo-build-release:
	cargo build --release --verbose

host: wasm-bindgen
	python -m SimpleHTTPServer 13137

host-release: wasm-bindgen-release
	python -m SimpleHTTPServer 13137

clean:
	cargo clean
