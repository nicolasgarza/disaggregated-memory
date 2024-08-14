fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running build.rs");
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["src/proto/memory_service.proto"], &["src/proto"])?;
    println!("Finished compiling protos");
    Ok(())
}
