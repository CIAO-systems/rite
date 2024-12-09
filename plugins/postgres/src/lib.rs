use import::Importer;
use importer::PostgresImporter;

mod importer;

#[no_mangle]
pub fn create_importer(_name: &str) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(PostgresImporter::new()))
}
