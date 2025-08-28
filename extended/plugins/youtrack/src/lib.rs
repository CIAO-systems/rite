use importers::{generic::YouTrackImporter, time::YouTrackImporterTime};

pub mod importers;

/// This functions create an importer for YouTrack data
///
#[no_mangle]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "time" => Ok(Box::new(YouTrackImporterTime::new())),
        _ => Ok(Box::new(YouTrackImporter::new())),
    }
}
