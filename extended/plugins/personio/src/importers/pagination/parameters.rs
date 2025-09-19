use personio_rs::personnel::apis::configuration::Configuration;
use tokio::runtime::Runtime;

use crate::importers::configuration::PersonioHeaders;

pub struct GeneralParameters<'a> {
    pub runtime: &'a Runtime,
    pub configuration: &'a Configuration,
    pub personio_headers: &'a PersonioHeaders,
}

impl<'a> GeneralParameters<'a> {
    pub(crate) fn new(
        runtime: &'a Runtime,
        configuration: &'a Configuration,
        personio_headers: &'a PersonioHeaders,
    ) -> Self {
        Self {
            runtime,
            configuration,
            personio_headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use personio_rs::personnel::apis::configuration::Configuration;
    use tokio::runtime::Runtime;

    use crate::importers::{
        configuration::PersonioHeaders, pagination::parameters::GeneralParameters,
    };

    #[test]
    fn test_new() -> Result<(), Box<dyn Error>> {
        let runtime = Runtime::new()?;
        let configuration = Configuration::new();
        let personio_headers = PersonioHeaders {
            partner_id: Some("partner-id".into()),
            app_id: Some("app-id".into()),
        };
        let params = GeneralParameters::new(&runtime, &configuration, &personio_headers);

        assert_eq!(params.personio_headers.partner_id, Some("partner-id".into()));
        assert_eq!(params.personio_headers.app_id, Some("app-id".into()));

        Ok(())
    }
}
