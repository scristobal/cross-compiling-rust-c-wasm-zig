use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new().file("some-c-code/gcd.c").compile("def");

    let bindings = bindgen::builder()
        .header("some-c-code/gcd.h")
        .allowlist_function("gcd")
        .generate()?;

    bindings.write_to_file("src/bindings.rs")?;

    println!("cargo:rerun-if-changed=some-c-code");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
