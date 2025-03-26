use import::Importer;
use model::BoxedError;
use personio_rs::personnel::{
    apis::employees_api::company_employees_get, models::EmployeesResponse,
};

use crate::importers::pagination::{self, PageResult, Paginator, parameters::GeneralParameters};

use super::EmployeesFilter;

pub struct EmployeesParameters<'a> {
    pub general: GeneralParameters<'a>,
    pub filter: &'a EmployeesFilter,
}

impl Importer for super::Employees {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref runtime) = self.general.runtime {
            let configuration = &self.general.get_personnel_configuration()?;
            let limit = self.general.limit.unwrap_or(10);

            let mut paginator = Paginator::new(limit, get_employees_page);

            let params = EmployeesParameters {
                general: GeneralParameters::new(
                    runtime,
                    configuration,
                    &self.general.personio_headers,
                ),
                filter: &self.filter,
            };

            paginator.fetch_all(&params, |page_data| {
                self.handle_employee_response(handler, page_data.clone())
            })?;
        }
        Ok(())
    }
}

pub fn get_employees_page<'a>(
    params: &EmployeesParameters,
    limit: i32,
    page: i32,
) -> Result<PageResult<EmployeesResponse>, BoxedError> {
    let result: Result<EmployeesResponse, BoxedError> = params.general.runtime.block_on(async {
        let offset = pagination::next_offset(limit, page);
        Ok(company_employees_get(
            params.general.configuration,
            params.general.personio_headers.partner_id.as_deref(), // x_personio_partner_id,
            params.general.personio_headers.app_id.as_deref(),     // x_personio_app_id,
            Some(limit),                                           // limit,
            Some(offset),                                          // offset,
            params.filter.email.as_deref(),                        // email,
            params.filter.attributes.clone(),                      // attributes,
            params.filter.updated_since.as_deref(),                // updated_since,
        )
        .await?)
    });

    Ok(PageResult::from(result?))
}
