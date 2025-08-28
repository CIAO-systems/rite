use model::{BoxedError, record::Record};

use crate::macros;

use super::configuration::GeneralConfiguration;

mod importer;
mod initializable;

pub struct Projects {
    general: GeneralConfiguration,
}

impl Projects {
    pub fn new() -> Self {
        Self {
            general: GeneralConfiguration::new(),
        }
    }

    fn handle_response(
        &self,
        response: personio_rs::personnel::models::CompanyAttendancesProjectsGet200Response,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), BoxedError> {
        if response.success.unwrap_or(false) {
            if let Some(data) = response.data {
                for project in data {
                    let mut record = Record::new();
                    macros::add_field_direct!(record, project, id);

                    if let Some(attributes) = &project.attributes {
                        macros::add_field_direct!(record, attributes, name);
                        macros::add_field_direct!(record, attributes, active);
                        macros::add_field_direct!(record, attributes, created_at);
                        macros::add_field_direct!(record, attributes, updated_at);
                    }

                    handler.handle_record(&mut record)?;
                }
            }

            Ok(())
        } else {
            Err("We got an project response, but it was unsuccessful".into())
        }
    }
}

#[cfg(test)]
mod tests;
