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
