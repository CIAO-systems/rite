use import::Importer;
use model::BoxedError;
use personio_rs::personnel::{
    apis::{configuration::Configuration, employees_api::company_employees_get},
    models::EmployeesResponse,
};
use tokio::runtime::Runtime;

use crate::importers::pagination::{self, PageResult, Paginator};

pub struct Parameters<'a> {
    pub runtime: &'a Runtime,
    pub configuration: &'a Configuration,
}

impl Importer for super::Employees {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref runtime) = self.runtime {
            let configuration = &self.get_personnel_configuration()?;
            let limit = self.limit.unwrap_or(10);

            let mut paginator = Paginator::new(limit, get_employees_page);
            let params = Parameters {
                runtime,
                configuration,
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
            None,         // x_personio_partner_id,
            None,         // x_personio_app_id,
            Some(limit),  // limit,
            Some(offset), // offset,
            None,         // email,
            None,         // attributes_left_square_bracket_right_square_bracket,
            None,         // updated_since,
        )
        .await?)
    });

    Ok(PageResult::from(result?))
}
