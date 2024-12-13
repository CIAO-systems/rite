use transformer::CommonTransformer;

pub mod transformer;

#[no_mangle]
pub fn create_transformer(
    _name: &str,
) -> Result<Box<dyn transform::Transformer>, Box<dyn std::error::Error>> {
    Ok(Box::new(CommonTransformer::new()))
}
