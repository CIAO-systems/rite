// Normally, you would include the protos with this:
// `tonic::include_proto!("ciao.time_tracking");`
// But since we use a custom out_dir (see build.rs), we need to use
// `include!()` with the path (relative to this file)

include!("../../target/generated/ciao/ciao.time_tracking.rs");

pub mod absences {
    include!("../../target/generated/ciao/ciao.time_tracking.absences.rs");
}

pub mod cost_center {
    include!("../../target/generated/ciao/ciao.time_tracking.cost_center.rs");
}

pub mod project {
    include!("../../target/generated/ciao/ciao.time_tracking.project.rs");

    pub mod task {
        include!("../../target/generated/ciao/ciao.time_tracking.project.task.rs");
    }
}

pub mod time_type {
    include!("../../target/generated/ciao/ciao.time_tracking.time_type.rs");

    pub mod group {
        include!("../../target/generated/ciao/ciao.time_tracking.time_type.group.rs");
    }
}
