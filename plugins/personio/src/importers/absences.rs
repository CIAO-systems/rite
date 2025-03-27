use filter::AbsencesFilter;
use import::RecordHandler;
use model::BoxedError;

use super::configuration::GeneralConfiguration;

mod filter;
mod importer;
mod initializable;

pub struct Absences {
    general: GeneralConfiguration,
    filter: AbsencesFilter,
}
impl Absences {
    pub(crate) fn new() -> Self {
        Self {
            general: GeneralConfiguration::new(),
            filter: AbsencesFilter::new(),
        }
    }

    fn handle_response(
        &self,
        handler: &mut dyn RecordHandler,
        page: personio_rs::personnel::models::AbsencePeriodsResponse,
    ) -> Result<(), BoxedError> {
        println!("{:#?}", page);
        Ok(())
    }
}
