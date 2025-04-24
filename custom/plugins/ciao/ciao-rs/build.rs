use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let basepath = "ciao-proto/proto";
    let out_dir = "target/generated/ciao";

    // create the out_dir
    fs::create_dir_all(out_dir)?;

    tonic_build::configure()
        // make sure the directory exists, before compiling
        .out_dir(out_dir)
        .build_server(true)
        .build_client(true)
        .build_transport(true)
        .compile_protos(
            &[
                format!("{}/ciao/accounts/service.proto", basepath),
                format!("{}/ciao/common/timestamp.proto", basepath),
                format!("{}/ciao/core/auth/service.proto", basepath),
                format!("{}/ciao/core/config/service.proto", basepath),
                format!("{}/ciao/devices/service.proto", basepath),
                format!("{}/ciao/time_tracking/service.proto", basepath),
                format!("{}/ciao/time_tracking/absences/service.proto", basepath),
                format!("{}/ciao/time_tracking/cost_center/service.proto", basepath),
                format!("{}/ciao/time_tracking/project/service.proto", basepath),
                format!("{}/ciao/time_tracking/project/task/service.proto", basepath),
                format!("{}/ciao/time_tracking/time_type/service.proto", basepath),
                format!("{}/ciao/time_tracking/time_type/group/service.proto", basepath),
            ],
            &[basepath],
        )?;
    Ok(())
}
