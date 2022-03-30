//extern crate bindgen;


use std::env;
use std::path::Path;

fn main() {
    //RUSTFLAGS="-Z sanitizer=address";
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).display());
}

