use model::Initializable;

const CFG_FILENAME: &str = "filename";
const CFG_EXPORT_OVERWRITE: &str = "overwrite";

pub struct CSV {
    filename: Option<String>,
    export_header_written: bool,
    export_override: bool,
}

impl CSV {
    pub(crate) fn new() -> Self {
        CSV {
            filename: None,
            export_header_written: false,
            export_override: false,
        }
    }
}

impl Initializable for CSV {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.filename = config.get(CFG_FILENAME);
            self.export_override = match config.get(CFG_EXPORT_OVERWRITE) {
                Some(value) => value.parse::<bool>()?,
                None => false,
            };
        }

        Ok(())
    }
}

mod exporter;
mod importer;
