use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("service_descriptor.bin"))
        .compile_protos(
            &["proto/rite/v1/service.proto"], // Path to your .proto file
            &["proto"],                  // Directory to search for imports
        )?;
    Ok(())
}
