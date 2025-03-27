pub mod importers;
mod macros;


/// This functions creates an importer for the Personio plugin
///
#[unsafe(no_mangle)]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "employees" => Ok(Box::new(importers::employees::Employees::new())),
        "projects" => Ok(Box::new(importers::projects::Projects::new())),
        "absences" => Ok(Box::new(importers::absences::Absences::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}
