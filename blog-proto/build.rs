//! Генерация Rust-кода из protobuf-контрактов блога.

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/blog.v1.proto");
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("blog_descriptor.bin"))
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/blog.v1.proto"], &["proto"])?;

    Ok(())
}
