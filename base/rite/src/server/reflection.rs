use tonic_reflection::server::Builder;

fn create_reflection_builder() -> Builder<'static> {
    Builder::configure().register_encoded_file_descriptor_set(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/service_descriptor.bin"
    )))
}

pub fn v1() -> Result<
    tonic_reflection::server::v1::ServerReflectionServer<
        impl tonic_reflection::server::v1::ServerReflection,
    >,
    tonic_reflection::server::Error,
> {
    create_reflection_builder().build_v1()
}

pub fn v1alpha() -> Result<
    tonic_reflection::server::v1alpha::ServerReflectionServer<
        impl tonic_reflection::server::v1alpha::ServerReflection,
    >,
    tonic_reflection::server::Error,
> {
    create_reflection_builder().build_v1alpha()
}
