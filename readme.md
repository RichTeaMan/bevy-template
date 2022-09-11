# Bevy Template

A template for Bevy 0.8 with Rapier.

## Running

the template can be run locally using Cargo:

```bash
cargo run
```

### Building WASM

[Web Assembly](https://webassembly.org/) allows this program to be run in a browser.

Bevy template builds WASM files to the docs directory so it can be hosted in Github Pages.

```bash
rustup target install wasm32-unknown-unknown && cargo install wasm-bindgen-cli && \
cargo build --all-features --target wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./docs/ --target web target/wasm32-unknown-unknown/debug/bevy_template.wasm && \
cp assets/ docs/. -r
```

The WASM build can also be tested locally, **run this in the docs folder**:

```bash
python3 -m http.server
```

Note that this doesn't work so well in Firefox due to HTTPS constraints. Chrome has a better time of it.

## Asset credits

Soil texture: https://www.deviantart.com/fabooguy/art/Dirt-Ground-Texture-Tileable-2048x2048-441212191.

Ambient nature noises courtesy of BurghRecords: https://www.youtube.com/watch?v=sexUXJPljH4.
