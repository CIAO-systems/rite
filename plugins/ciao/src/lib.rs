use importers::{devices::CiaoDevices, projects::CiaoProjects};

pub mod connection;
pub mod config;
pub mod importers;

/// This functions create an importer for CIAO data
///
#[no_mangle]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "devices" => Ok(Box::new(CiaoDevices::new())),
        "projects" => Ok(Box::new(CiaoProjects::new())),
        _ => Err("Not implemented".into()),
    }
}