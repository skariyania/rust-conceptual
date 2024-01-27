use std::io::Result;

fn main() -> Result<()> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    tonic_build::compile_protos("proto/health.proto")?;
    Ok(())
}
