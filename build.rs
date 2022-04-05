extern crate bindgen;


use std::env;
use std::path::Path;


fn main() {
    //RUSTFLAGS="-Z sanitizer=address";
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).display());
    
    let bindings = bindgen::Builder::default()
        .header("iir_blur.h")
        .generate()
        .expect("Unable to generate bindings");

   	bindings.write_to_file("outTEST.rs").expect("Couldn't write bindings");
}
