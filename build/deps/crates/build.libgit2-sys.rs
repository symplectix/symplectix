// https://github.com/rust-lang/git2-rs/blob/master/libgit2-sys/build.rs

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }
}
