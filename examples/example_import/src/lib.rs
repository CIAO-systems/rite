use import::Importer;
use text::TextFileImporter;

pub mod text;

#[no_mangle]
pub fn create_importer(name: &str) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
    match name {
        "text" => Ok(Box::new(TextFileImporter::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}
