pub mod csv;

/// This functions creates an importer for CSV
///
#[no_mangle]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(csv::CSV::new()))
}

/// This functions creates an importer for CSV
///
#[no_mangle]
pub fn create_exporter(
    _name: &str,
) -> Result<Box<dyn model::export::Exporter>, Box<dyn std::error::Error>> {
    Ok(Box::new(csv::CSV::new()))
}