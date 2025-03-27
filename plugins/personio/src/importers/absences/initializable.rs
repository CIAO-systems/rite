use model::Initializable;

use crate::importers::{absences::filter::AbsencesFilter, configuration::GeneralConfiguration};

impl Initializable for super::Absences {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.general = GeneralConfiguration::load(&config)?;
            self.filter = AbsencesFilter::load(&config)?;
        }
        Ok(())
    }
}
