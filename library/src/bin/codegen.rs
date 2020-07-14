use std::env::set_current_dir;
use std::process::Command;

fn main() {
    genfut::genfut(genfut::Opt {
        name: "neptune-triton".to_string(),
        file: std::path::PathBuf::from("poseidon.fut"),
        author: "porcuquine@gmail.com".to_string(),
        version: "1.1.1".to_string(),
        description: "GPU implementation of neptune-compatible Poseidon hashing.".to_string(),
        license: "MIT OR Apache-2.0".to_string(),
    });

    let toolchain_raw = std::fs::read_to_string("../rust-toolchain").expect(
        "toolchain not found. codegen should be run from /neptune-triton/library directory.",
    );
    set_current_dir("neptune-triton").unwrap();

    let toolchain = toolchain_raw[..].trim_end();

    Command::new("cargo")
        .arg(format!("+{}", toolchain))
        .arg("fmt")
        .output()
        .expect("failed to format");
}
