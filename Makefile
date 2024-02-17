

## Try out in WASI


# ensure wasm32-wasi target is installed
rustup target add wasm32-wasi

# cross compile to WASI
cargo zigbuild --target=wasm32-wasi --release

# try it out, requires wasm3
wasm3 target/wasm32-wasi/release/rust-ffi-playground.wasm


## Compile for web

# cross compile to WASM
cargo zigbuild --target=wasm32-unknown-unknown --release

# generate JS and TS bindings to WASM code
wasm-bindgen target/wasm32-unknown-unknown/release/rust-ffi-playground.wasm --out-dir ./bin --target web

# include the file in HTML with
 <script type="module">
    import init from "./bin/rust-ffi-playground.js";
    init().then( () => console.log( "WASM Loaded" ) );
 </script>

 # or use a WASM plugin like vite-wasm-plugin (check that is the right name)


## Use cargo_zigbuild directly in build.rs

# check commented code
