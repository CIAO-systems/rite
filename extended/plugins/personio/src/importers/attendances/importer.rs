use model::import::Importer;
use model::BoxedError;
use personio_rs::personnel::{
    apis::attendances_api::company_attendances_get, models::AttendancePeriodsResponse,
};

use crate::importers::pagination::{self, PageResult, Paginator, parameters::GeneralParameters};

use super::AttendancesFilter;

pub struct AttendancesParameters<'a> {
    general: GeneralParameters<'a>,
    filter: &'a AttendancesFilter,
}

impl Importer for super::Attendances {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref runtime) = self.general.runtime {
            let configuration = &self.general.get_personnel_configuration()?;
            let limit = self.general.limit.unwrap_or(10);
            let mut paginator = Paginator::new(limit, get_attendance_page);

            let params = AttendancesParameters {
                general: GeneralParameters::new(
                    runtime,
                    configuration,
                    &self.general.personio_headers,
                ),
                filter: &self.filter,
            };

            paginator.fetch_all(&params, |page_data| {
                self.handle_attendance_response(handler, page_data.clone())
            })?;
        }

        Ok(())
    }
}

pub fn get_attendance_page<'a>(
    params: &AttendancesParameters,
    limit: i32,
    page: i32,
) -> Result<PageResult<AttendancePeriodsResponse>, BoxedError> {
    let result: Result<AttendancePeriodsResponse, BoxedError> =
        params.general.runtime.block_on(async {
            let _offset = pagination::next_offset(limit, page);

            let attendances = company_attendances_get(
                params.general.configuration,
                params.filter.start_date.clone(),
                params.filter.end_date.clone(),
                params.general.personio_headers.partner_id.as_deref(), // x_personio_partner_id,
                params.general.personio_headers.app_id.as_deref(),     // x_personio_app_id,
                params.filter.updated_from.as_deref(),                 // updated_from,
                params.filter.updated_to.as_deref(),                   // updated_to,
                params.filter.include_pending,                         // include_pending,
                params.filter.employees.clone(),                       // employees,
                // See https://ciao-systems.youtrack.cloud/issue/RIT-32/Attendance-import#focus=Comments-4-285.0-0
                None, // Some(limit),                                           // limit,
                None, // Some(offset),                                          // offset,
            )
            .await;

            Ok(attendances?)
        });

    Ok(PageResult::from(result?))
}
