use importers::Faker;

pub mod importers;

/// This functions creates an importer for the fake record generator
///
#[no_mangle]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(Faker::new()))
}