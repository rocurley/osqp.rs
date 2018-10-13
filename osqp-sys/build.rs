use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = "/home/roger/git/osqp_wasm/build/out";
    //println!(r#"cargo:rustc-cfg=feature="osqp_dlong""#);
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=osqp");
}
