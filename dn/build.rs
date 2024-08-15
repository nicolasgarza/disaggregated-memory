fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running build.rs");
    tonic_build::compile_protos("src/proto/memory.proto")?;
    println!("Finished compiling protos");
    Ok(())
}
