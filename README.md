# From Rust + C to Web Assembly with the help of Zig

How to cross-compile a Rust project with C dependencies into WASM and WASI using Zig

## Create a sample project

 TODO: Write main code in Rust plus auxiliary library in C

## Quick and dirty, try out in WASI using CLIs

```bash
rustup target add wasm32-wasi # ensure wasm32-wasi target is installed `
cargo zigbuild --target=wasm32-wasi --release # cross compile to WASI
wasm3 target/wasm32-wasi/release/rust-ffi-playground.was # try it out, requires wasm3 TODO: add install ref.
```

## Cross compile for web (using CLIs)

```bash
cargo zigbuild --target=wasm32-unknown-unknown --release # cross compile to WASM
wasm-bindgen target/wasm32-unknown-unknown/release/rust-ffi-playground.wasm --out-dir ./bin --target web # generate JS and TS bindings to WASM code
```

manually include the file in HTML with

```html
 <script type="module">
    import init from "./bin/rust-ffi-playground.js";
    init().then( () => console.log( "WASM Loaded" ) );
 </script>
```

or use a WASM plugin like vite-wasm-plugin  TODO: add reference and link

## Cross compile for the web (using `builder.rs`)

Use `cargo_zigbuild` and `bindgen` directly in `build.rs`, check commented code on `build.rs`   TODO: explain the changes

## Link to a system library

## References

Official cargo reference: <https://doc.rust-lang.org/cargo/reference/build-script-examples.html>
Zig cross compilation:
