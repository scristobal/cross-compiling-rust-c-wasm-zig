# Crosscompile a Rust project with C dependencies into Web Assembly (WASM and WASI) using Zig

Easy way using [rust-bindgen](https://github.com/rust-lang/rust-bindgen) and [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) CLI tools.

### Generate Rust bindings for the C code

We just need the C/C++ header files wit the definitions and [rust-bindgen](https://github.com/rust-lang/rust-bindgen) will generate the Rust FFI bindings.

```bash
bindgen vendor/gcd.h -o src/bindings.rs # generate Rust FFI bindings for gcd.h
```

### Quick WASI try out

Using [zigbuild](https://github.com/rust-cross/cargo-zigbuild) we can cross compile to WASI 

```bash
rustup target add wasm32-wasip1 # make sure wasm32-wasi target is installed 
cargo zigbuild --target=wasm32-wasip1 --release # cross compile to WASI, release flag is optional
```

> [!warning]
> Previously the target `wasm32-wasip1` was `wasm32-wasi` but it is now being deprecated, still you might want to use it even if you [get some warnings](https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html#renaming-wasm32-wasi-to-wasm32-wasip1).

we can try it with [wasm3](https://github.com/wasm3/wasm3) engine 

```bash
wasm3 target/wasm32-wasip1/release/rust-ffi-playground.wasm # try it out, requires wasm3 
```

or [wasmi](https://github.com/wasmi-labs/wasmi)

```bash
wasmi_cli target/wasm32-wasip1/release/rust-ffi-playground.wasm # run it with wasmi runtime
```


### Cross compile for web (WASM)

Generate code for WASM with [zigbuild](https://github.com/rust-cross/cargo-zigbuild), and then use [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to generate the js/ts bindings to the WASM code.

```bash
cargo zigbuild --target=wasm32-unknown-unknown --release # cross compile to WASM, release flag is optional
wasm-bindgen target/wasm32-unknown-unknown/release/rust-ffi-playground.wasm --out-dir ./dist --target web # generate JS and TS FFI bindings into WASM code
```

To try it, manually include the script tag to load and initialize the wasm module

```html
<script type="module">
  import init from "./bin/rust-ffi-playground.js";
  init().then(() => console.log("WASM Loaded"));
</script>
```

or use a WASM plugin like [`vite-plugin-wasm`](https://www.npmjs.com/package/vite-plugin-wasm) or use [Trunk](https://trunkrs.dev/)

Note: As in this [github issue](https://github.com/rustwasm/team/issues/291#issuecomment-644946504) it can be compiled to `wasm32-unknown-emscripten`. Can we then use `wasm-bindgen` to generate the bindings?

Same [example](https://github.com/rustwasm/team/issues/291#issuecomment-645492619) but using `wasm-pack`, hence `wasm-bindgen` instead

### Going further

Not a trivial project, eg. building a native library, link to a system library...

Replace CLI commands with a simple `Makefile` or (even  better) a `builder.rs` file, eg. use `cargo_zigbuild` and `bindgen` directly in `build.rs` by replacing `cc` with `zigbuild`

```rust
use cargo_zigbuild::Zig::Cc;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new().file("vendor/def.c").compile("def");

    let out_dir = env::var("OUT_DIR").unwrap();

    let cc = Cc {
        args: vec![
            format!("vendor/def.c"),
            "-c".to_string(),
            "-o".to_string(),
            format!("{}/def.o", out_dir),
        ],
    };

    cc.execute().expect("Failed to compile def.c");

    let ar = cargo_zigbuild::Zig::Ar {
        args: vec![
            "crus".to_string(),
            format!("{}/libdef.a", out_dir),
            format!("{}/def.o", out_dir),
        ],
    };

    ar.execute().expect("Failed to create def.a");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=def");

    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
```

## References

- [The `cc-rs` project](https://crates.io/crates/cc)
- [Official cargo reference](https://doc.rust-lang.org/cargo/reference/build-script-examples.html)
- Zig cross compilation
- [Bindgen tutorial](https://rust-lang.github.io/rust-bindgen/tutorial-3.html)
- [The embedded Rust book](https://docs.rust-embedded.org/book/interoperability/c-with-rust.html)
- [Shrinking `.wasm` code size](https://rustwasm.github.io/docs/book/reference/code-size.html)

## Issues

wasm-bindgen targets `wasm32-unknown-unknown` and `wasi-unknown` do not (fully) support C-ABI, only older targets like `wasm32-unknown-emscripten`.

See [comment](https://github.com/rustwasm/team/issues/291#issuecomment-645482430), [comment](https://github.com/rustwasm/team/issues/291#issuecomment-645494771) and [documentation PR](https://github.com/rustwasm/wasm-bindgen/pull/2209)

There is an experimental flag `--Z wasm_c_abi=spec` that [circumvents this limitation](https://github.com/rustwasm/team/issues/291#issuecomment-2138201722)

## Other tools

- [c2rust](https://github.com/immunant/c2rust) - C to Rust translator produces `unsafe` Rust code from C99-compilant C code. It does not support cross compilation, but maybe it can with the help of Zig.

From their website:

> C source code is parsed and typechecked using clang before being translated by our tool.

would it be possible to use it with Zig as a drop-in replacement for clang?

From their README:

> I translated code on platform X, but it didn't work correctly on platform Y.
> We run the C preprocessor before translation to Rust. This specializes the code to the host platform. For this reason, we do not support cross compiling translated code at the moment.
> What platforms can C2Rust be run on?
> The translator and refactoring tool support both macOS and Linux. Other features, such as cross checking the functionality between C and Rust code, are currently limited to Linux hosts.

## Utils

- <https://wasm-feature-detect.surma.technology/> [source](https://github.com/GoogleChromeLabs/wasm-feature-detect)

---

## Remarks

> In general, a `lib<name>.so` or `lib<name>.a` should be referenced in the build file by `<name>`.
