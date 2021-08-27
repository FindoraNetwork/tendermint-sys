use std::env;
use std::process::Command;
// use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum LevelDB {
    Cleveldb,
    Goleveldb,
}

fn main() {
    println!("cargo:rerun-if-changed=tmgo/app.go");
    println!("cargo:rerun-if-changed=tmgo/main.go");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=script/build.sh");
    let out_dir = env::var("OUT_DIR").unwrap();
    let code_dir = env::current_dir().unwrap();
    let go_dir = code_dir.join("tmgo");

    let leveldb = match env::var("CARGO_FEATURE_CLEVELDB") {
        Ok(_) => LevelDB::Cleveldb,
        Err(_) => LevelDB::Goleveldb,
    };

    // get libffi_slim.a

    let mut command = Command::new("../scripts/build.sh");
    if leveldb == LevelDB::Cleveldb {
        command.arg("cleveldb");
    } else {
        command.arg("goleveldb");
    }
    command.arg(&format!("{}/libtmgo.a", out_dir));
    command.current_dir(go_dir);

    let status = command.status().unwrap();

    assert!(status.success());

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=tmgo");

    let target = env::var("TARGET").unwrap();
    if target == "x86_64-apple-darwin" {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }
}
