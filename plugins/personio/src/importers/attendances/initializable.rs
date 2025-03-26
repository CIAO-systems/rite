use model::Initializable;

use crate::importers::configuration::GeneralConfiguration;

use super::AttendancesFilter;


impl Initializable for super::Attendances {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.general = GeneralConfiguration::load(&config)?;
            self.filter = AttendancesFilter::load(&config)?;
        }
        Ok(())
    }
}
