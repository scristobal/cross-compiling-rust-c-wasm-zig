# How to cross-compile a Rust project with C dependencies into WASM and WASI using Zig

## Try out in WASI

```bash

rustup target add wasm32-wasi #ensure wasm32-wasi target is installed `

cargo zigbuild --target=wasm32-wasi --release #cross compile to WASI

wasm3 target/wasm32-wasi/release/rust-ffi-playground.was # try it out, requires wasm3 TODO: add install ref.
```

## Compile for web

```bash
cargo zigbuild --target=wasm32-unknown-unknown --release # cross compile to WASM



wasm-bindgen target/wasm32-unknown-unknown/release/rust-ffi-playground.wasm --out-dir ./bin --target web# generate JS and TS bindings to WASM code

```

manually include the file in HTML with

```html
 <script type="module">
    import init from "./bin/rust-ffi-playground.js";
    init().then( () => console.log( "WASM Loaded" ) );
 </script>
```

or use a WASM plugin like vite-wasm-plugin [//]: # (check that is the right name)

## Use cargo_zigbuild directly in build.rs

check commented code on `build.rs`
