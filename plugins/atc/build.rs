use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = match env::current_dir()?.to_str() {
        Some(dir) => String::from(dir),
        None => String::from(""),
    };

    let basepath = format!("{}/bin/proto", base);

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .build_transport(true)
        .compile_protos(
            &[
                &format!("{}/v1/reflection.proto", basepath),
                &format!("{}/v1alpha/reflection.proto", basepath),
            ],
            &[basepath],
        )?;
    Ok(())
}
