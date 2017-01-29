use std::path::Path;
use std::env;

fn main() {
    println!("cargo:rustc-link-lib=static=ta_common_cdd");
    println!("cargo:rustc-link-lib=static=ta_abstract_cdd");
    println!("cargo:rustc-link-lib=static=ta_func_cdd");
    println!("cargo:rustc-link-lib=static=ta_libc_cdd");
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}", Path::new(&dir).join("ta-lib").join("lib").display());
}