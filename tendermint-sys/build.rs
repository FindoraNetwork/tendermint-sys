use std::env;
use std::process::Command;
use std::path::Path;

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
    println!("cargo:rerun-if-changed=script/make-go-root.sh");
    let out_dir = env::var("OUT_DIR").unwrap();
    let code_dir = env::current_dir().unwrap();
    let go_dir = code_dir.join("tmgo");

    let go_rootfs = format!("{}/go-rootfs", out_dir);

    if Path::new(&go_rootfs).exists() {
        let mut command = Command::new("../scripts/clean-go-root.sh");
        command.arg(&go_rootfs);
        command.current_dir(go_dir.clone());
        let status = command.status()
            .unwrap();
        assert!(status.success());
    }

    // prepare go rootfs
    let mut command = Command::new("../scripts/make-go-root.sh");
    command.arg(&go_rootfs);
    command.arg(&format!("{}/musl.patch", go_dir.to_str().unwrap()));
    command.current_dir(go_dir.clone());

    let status = command.status()
        .unwrap();
    assert!(status.success());


    let leveldb = match env::var("CARGO_FEATURE_CLEVELDB") {
        Ok(_) => LevelDB::Cleveldb,
        Err(_) => LevelDB::Goleveldb,
    };

    let mut command = Command::new("../scripts/build.sh");
    if leveldb == LevelDB::Cleveldb {
        command.arg("cleveldb");
    } else {
        command.arg("goleveldb");
    }
    command.arg(&format!("{}/libtmgo.a", out_dir));
    command.arg(&go_rootfs);
    command.current_dir(go_dir);

    let status = command.status().unwrap();

    assert!(status.success());

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=tmgo");

    if leveldb == LevelDB::Cleveldb {
        println!("cargo:rustc-link-lib=leveldb");
    }

    let target = env::var("TARGET").unwrap();
    if target == "x86_64-apple-darwin" {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }
}
