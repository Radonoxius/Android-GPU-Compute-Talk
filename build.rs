use std::env::var;

fn main() {
    println!("cargo::rustc-link-arg=--sysroot={}", var("NDK_SYSROOT").unwrap());
    println!("cargo::rustc-link-arg=-resource-dir={}", var("CC_RES_DIR").unwrap());
}