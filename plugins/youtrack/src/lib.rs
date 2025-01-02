use importers::YouTrackImporter;

pub mod importers;

/// This functions create an importer for YouTrack data
/// 
#[no_mangle]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(YouTrackImporter::new()))
}
