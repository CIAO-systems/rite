use model::import::Importer;
use model::BoxedError;
use personio_rs::personnel::{
    apis::projects_api::company_attendances_projects_get,
    models::CompanyAttendancesProjectsGet200Response,
};

impl Importer for super::Projects {
    fn read(&mut self, handler: &mut dyn model::import::RecordHandler) -> Result<(), BoxedError> {
        if let Some(ref runtime) = self.general.runtime {
            let response: Result<CompanyAttendancesProjectsGet200Response, BoxedError> = runtime
                .block_on(async {
                    let configuration = &self.general.get_personnel_configuration()?;
                    let projects = company_attendances_projects_get(
                        &configuration,
                        self.general.personio_headers.partner_id.as_deref(),
                        self.general.personio_headers.app_id.as_deref(),
                    )
                    .await?;
                    Ok(projects)
                });

            return match response {
                Ok(response) => self.handle_response(response, handler),
                Err(e) => Err(e),
            };
        }
        Ok(())
    }
}
