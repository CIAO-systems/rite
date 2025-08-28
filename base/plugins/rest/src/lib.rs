use importer::RESTImporter;

mod importer;

#[no_mangle]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(RESTImporter::new()))
}
