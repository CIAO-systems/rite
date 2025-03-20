pub mod employees {
    use import::Importer;
    use model::{BoxedError, Initializable, record::Record};
    use personio_rs::{
        auth::login,
        personnel::{
            apis::{configuration::Configuration, employees_api::company_employees_get},
            models::EmployeesResponse,
        },
    };
    use tokio::runtime::Runtime;

    mod macros;

    const CFG_CLIENT_ID: &str = "client_id";
    const CFG_CLIENT_SECRET: &str = "client_secret";


    pub struct Employees {
        token: Option<String>,
        runtime: Option<Runtime>,
    }

    impl Employees {
        pub(crate) fn new() -> Self {
            Self {
                token: None,
                runtime: None,
            }
        }

        /// Get the Configuration with the `bearer_access_token`
        fn get_personnel_configuration(&self) -> Result<Configuration, BoxedError> {
            if let Some(ref token) = self.token {
                let mut configuration = Configuration::new();
                configuration.bearer_access_token = Some(token.clone());
                Ok(configuration)
            } else {
                Err("No valid token stored".into())
            }
        }

        /// Iterate the EmployeesResponse and call the record handler.
        fn handle_employee_response(
            &self,
            handler: &mut dyn import::RecordHandler,
            employee_response: EmployeesResponse,
        ) -> Result<(), BoxedError> {
            if employee_response.success {
                for data in employee_response.data {
                    if let Some(attr) = data.attributes {
                        // add all attributes to a record
                        let mut record = self.create_record(attr)?;
                        handler.handle_record(&mut record)?;
                    }
                }
                Ok(())
            } else {
                Err("We got an employee response, but it was unsuccessful".into())
            }
        }

        fn create_record(
            &self,
            attr: Box<personio_rs::personnel::models::Employee>,
        ) -> Result<Record, BoxedError> {
            let mut record = Record::new();
            let fields = record.fields_as_mut();

            macros::add_field!(fields, attr, id);
            macros::add_field!(fields, attr, email);
            macros::add_field!(fields, attr, first_name);
            macros::add_field!(fields, attr, gender);
            macros::add_field!(fields, attr, last_name);
            macros::add_field!(fields, attr, preferred_name);
            macros::add_field!(fields, attr, status);
            macros::add_field!(fields, attr, created_at);

            Ok(record)
        }
    }

    impl Initializable for Employees {
        fn init(
            &mut self,
            config: Option<model::xml::config::Configuration>,
        ) -> Result<(), BoxedError> {
            if let Some(config) = config {
                if let Some(client_id) = config.get(CFG_CLIENT_ID) {
                    if let Some(client_secret) = config.get(CFG_CLIENT_SECRET) {
                        let runtime = Runtime::new()?;
                        let result: Result<String, BoxedError> =
                            runtime.block_on(async { Ok(login(client_id, client_secret).await?) });
                        match result {
                            Ok(token) => {
                                // We have a valid token now, store it and the tokio runtime
                                self.token = Some(token);
                                self.runtime = Some(runtime);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
            Ok(())
        }
    }

    impl Importer for Employees {
        fn read(
            &mut self,
            handler: &mut dyn import::RecordHandler,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let configuration = self.get_personnel_configuration()?;
            if let Some(ref runtime) = self.runtime {
                let result: Result<EmployeesResponse, BoxedError> = runtime.block_on(async {
                    Ok(company_employees_get(
                        &configuration,
                        None, // x_personio_partner_id,
                        None, // x_personio_app_id,
                        None, // limit,
                        None, // offset,
                        None, // email,
                        None, // attributes_left_square_bracket_right_square_bracket,
                        None, // updated_since,
                    )
                    .await?)
                });

                self.handle_employee_response(handler, result?)?;
            }
            Ok(())
        }
    }
}
