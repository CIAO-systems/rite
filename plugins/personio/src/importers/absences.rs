use super::configuration::GeneralConfiguration;

mod importer;
mod initializable;

pub struct Absences {
    general: GeneralConfiguration,
}
impl Absences {
    pub(crate) fn new() -> Self {
        Self {
            general: GeneralConfiguration::new(),
        }
    }
}
