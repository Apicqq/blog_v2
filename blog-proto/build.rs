//! Генерация Rust-кода из protobuf-контрактов блога.

fn main() {
    println!("cargo:rerun-if-changed=proto/blog.v1.proto");

    tonic_prost_build::compile_protos("proto/blog.v1.proto")
        .expect("не удалось сгенерировать Rust-код из protobuf-контрактов");
}
