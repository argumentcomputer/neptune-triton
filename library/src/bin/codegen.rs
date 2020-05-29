fn main() {
    genfut::genfut(genfut::Opt {
        name: "neptune-triton".to_string(),
        file: std::path::PathBuf::from("poseidon.fut"),
        author: "porcuquine@gmail.com".to_string(),
        version: "1.0.0".to_string(),
        description: "GPU implementation of neptune-compatible Poseidon hashing.".to_string(),
        license: "MIT OR Apache-2.0".to_string(),
    })
}
