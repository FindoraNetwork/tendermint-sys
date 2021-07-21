use std::env;
use std::process::Command;
// use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=tmgo/app.go");
    println!("cargo:rerun-if-changed=tmgo/main.go");
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = env::var("OUT_DIR").unwrap();
    let code_dir = env::current_dir().unwrap();
    let go_dir = code_dir.join("tmgo");

    let status = Command::new("../scripts/build.sh")
        .arg(&format!("{}/libtmgo.a", out_dir))
        .current_dir(go_dir)
        .status()
        .unwrap();

    assert!(status.success());

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=tmgo");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=Security");
}
