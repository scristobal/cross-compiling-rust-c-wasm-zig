use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new().file("vendor/def.c").compile("def");

    // let bindings = bindgen::Builder::default()
    //     .header("vendor/def.h")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings");

    // bindings
    //     .write_to_file("src/bindings.rs")
    //     .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
