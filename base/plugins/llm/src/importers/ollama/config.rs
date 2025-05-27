use model::Initializable;

use super::OllamaImporter;

impl Initializable for OllamaImporter {
    fn init(
        &mut self,
        _config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {

        Ok(())
    }
}
