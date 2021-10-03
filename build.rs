use std::env;
use std::fs;
use std::path::Path;

// WASM builder
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
