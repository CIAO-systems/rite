use importers::{generic::YouTrackImporter, time::YouTrackImporterTime};

pub mod importers;

/// This functions create an importer for YouTrack data
///
#[unsafe(no_mangle)]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "time" => Ok(Box::new(YouTrackImporterTime::new())),
        _ => Ok(Box::new(YouTrackImporter::new())),
    }
}

#[cfg(test)]
mod tests {
    use crate::create_importer;

    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    fn test_importer_with(name: &str) {
        let importer = create_importer(name);
        assert!(importer.is_ok());
        let importer = importer.unwrap();
        assert_eq!(
            type_of(&importer),
            "alloc::boxed::Box<dyn model::import::Importer>"
        );
    }

    #[test]
    fn test_create_importer() {
        test_importer_with("time");
        test_importer_with("any");
    }
}
