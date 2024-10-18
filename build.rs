fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the proto file and generate the Rust code
    tonic_build::compile_protos("api/api.proto")?;

    Ok(())
}