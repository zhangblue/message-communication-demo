fn main() -> anyhow::Result<()> {
    tonic_build::compile_protos("proto/communication_api.proto")?;
    Ok(())
}
