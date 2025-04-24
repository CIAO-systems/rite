// Normally, you would include the protos with this:
// `tonic::include_proto!("ciao.common");`
// But since we use a custom out_dir (see build.rs), we need to use
// `include!()` with the path (relative to this file)
include!("../../target/generated/ciao/ciao.accounts.rs");
