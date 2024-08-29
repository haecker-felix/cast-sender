use std::io::Result;

fn main() -> Result<()> {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    prost_build::compile_protos(
        &["proto/cast_channel.proto", "proto/authority_keys.proto"],
        &[""],
    )?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./proto/cast_channel.proto");
    println!("cargo:rerun-if-changed=./proto/authority_keys.proto");

    Ok(())
}
