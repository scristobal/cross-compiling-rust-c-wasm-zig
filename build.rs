// use cargo_zigbuild::Zig::Cc;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new().file("vendor/def.c").compile("def");

    // Cc {
    //     args: vec!["vendor/def.c".to_string()],
    // };

    println!("cargo:rerun-if-changed=vendor.c");

    Ok(())
}
