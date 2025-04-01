use import::Importer;
use model::BoxedError;
use personio_rs::personnel::{
    apis::absences_api::company_time_offs_get, models::AbsencePeriodsResponse,
};

use crate::importers::pagination::{self, PageResult, Paginator, parameters::GeneralParameters};

use super::filter::AbsencesFilter;

pub struct AbsencesParameters<'a> {
    general: GeneralParameters<'a>,
    filter: &'a AbsencesFilter,
}

impl Importer for super::Absences {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref runtime) = self.general.runtime {
            let configuration = &self.general.get_personnel_configuration()?;
            let limit = self.general.limit.unwrap_or(10);
            let mut paginator = Paginator::new(limit, get_absences_page);

            let params = AbsencesParameters {
                general: GeneralParameters::new(
                    runtime,
                    configuration,
                    &self.general.personio_headers,
                ),
                filter: &self.filter,
            };

            paginator.fetch_all(&params, |page_data| {
                self.handle_response(handler, page_data.clone())
            })?;
        }
        Ok(())
    }
}

pub fn get_absences_page<'a>(
    params: &AbsencesParameters,
    limit: i32,
    page: i32,
) -> Result<PageResult<AbsencePeriodsResponse>, BoxedError> {
    let result: Result<AbsencePeriodsResponse, BoxedError> =
        params.general.runtime.block_on(async {
            let offset = pagination::next_offset(limit, page);

            let absences = company_time_offs_get(
                params.general.configuration,
                params.general.personio_headers.partner_id.as_deref(), // x_personio_partner_id,
                params.general.personio_headers.app_id.as_deref(),     // x_personio_app_id,
                params.filter.start_date.clone(),                      // start_date,
                params.filter.end_date.clone(),                        // end_date,
                params.filter.updated_from.clone(),                    // updated_from,
                params.filter.updated_to.clone(),                      // updated_to,
                params.filter.employees.clone(),                       // employees,
                Some(limit),                                           // limit,
                Some(offset),                                          // offset,
            )
            .await;

            if absences.is_err() {
                let e = absences.as_ref().unwrap_err();
                let text = format!("Error:\n{:#?}\nFilter:\n{:#?}", e, params.filter);
                eprintln!("{text}");
                log::error!("{text}");
            }
            Ok(absences?)
        });

    Ok(PageResult::from(result?))
}
