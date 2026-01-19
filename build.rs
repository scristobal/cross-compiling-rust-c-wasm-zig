use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new().file("some-c/gcd.c").compile("gcd");

    // Force parsing for x86_64-linux since the C header is platform-agnostic
    // but clang may fail to parse when targeting wasm without proper sysroot
    let bindings = bindgen::builder()
        .header("some-c/gcd.h")
        .use_core()
        .clang_arg("--target=x86_64-unknown-linux-gnu")
        .generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    println!("cargo:rerun-if-changed=some-c");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
