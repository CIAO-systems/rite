use import::Importer;
use model::BoxedError;
use personio_rs::personnel::{
    apis::{configuration::Configuration, employees_api::company_employees_get},
    models::EmployeesResponse,
};
use tokio::runtime::Runtime;

use crate::importers::{
    configuration::PersonioHeaders,
    pagination::{self, PageResult, Paginator},
};

use super::Filter;

pub struct Parameters<'a> {
    pub runtime: &'a Runtime,
    pub configuration: &'a Configuration,
    pub filter: &'a Filter,
    pub personio_headers: &'a PersonioHeaders,
}

impl Importer for super::Employees {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref runtime) = self.general.runtime {
            let configuration = &self.general.get_personnel_configuration()?;
            let limit = self.limit.unwrap_or(10);

            let mut paginator = Paginator::new(limit, get_employees_page);
            let params = Parameters {
                runtime,
                configuration,
                filter: &self.filter,
                personio_headers: &self.general.personio_headers,
            };

            paginator.fetch_all(&params, |page_data| {
                self.handle_employee_response(handler, page_data.clone())
            })?;
        }
        Ok(())
    }
}

pub fn get_employees_page<'a>(
    params: &Parameters,
    limit: i32,
    page: i32,
) -> Result<PageResult<EmployeesResponse>, BoxedError> {
    let result: Result<EmployeesResponse, BoxedError> = params.runtime.block_on(async {
        let offset = pagination::next_offset(limit, page);
        Ok(company_employees_get(
            params.configuration,
            params.personio_headers.partner_id.as_deref(), // x_personio_partner_id,
            params.personio_headers.app_id.as_deref(),     // x_personio_app_id,
            Some(limit),                                   // limit,
            Some(offset),                                  // offset,
            params.filter.email.as_deref(),                // email,
            params.filter.attributes.clone(),              // attributes,
            params.filter.updated_since.as_deref(),        // updated_since,
        )
        .await?)
    });

    Ok(PageResult::from(result?))
}
