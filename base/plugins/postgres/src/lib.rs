use exporter::PostgresExporter;
use importer::PostgresImporter;

mod common;
mod exporter;
mod importer;

#[no_mangle]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(PostgresImporter::new()))
}

#[no_mangle]
pub fn create_exporter(
    _name: &str,
) -> Result<Box<dyn export::Exporter>, Box<dyn std::error::Error>> {
    Ok(Box::new(PostgresExporter::new()))
}
