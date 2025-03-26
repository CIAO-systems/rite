use model::{BoxedError, Initializable};

use crate::importers::configuration::GeneralConfiguration;

use super::FLAG_SALARY;

const CFG_OPTIONS_LIMIT: &str = "options.limit";
const CFG_FILTER_EMAIL: &str = "filter.email";
const CFG_FILTER_UPDATED_SINCE: &str = "filter.updated_since";
const CFG_FILTER_ATTRIBUTES: &str = "filter.attributes";

impl Initializable for super::Employees {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        if let Some(config) = config {
            self.general = GeneralConfiguration::load(&config)?;

            // read flags
            if let Some(salary) = config.get(FLAG_SALARY) {
                if let Ok(salary) = salary.parse::<bool>() {
                    self.flags.insert(String::from(FLAG_SALARY), salary);
                }
            }

            // read options
            if let Some(limit) = config.get(CFG_OPTIONS_LIMIT) {
                if let Ok(limit) = limit.parse::<i32>() {
                    self.limit = Some(limit);
                }
            }

            // read filters
            if let Some(email) = config.get(CFG_FILTER_EMAIL) {
                self.filter.email = Some(email);
            }

            if let Some(updated_since) = config.get(CFG_FILTER_UPDATED_SINCE) {
                self.filter.updated_since = Some(updated_since);
            }

            if let Some(attributes) = config.get(CFG_FILTER_ATTRIBUTES) {
                self.filter.set_attributes(attributes);
            }
        }
        Ok(())
    }
}
