use std::collections::HashMap;

use import::Importer;
use model::{BoxedError, Initializable, field::add_field, record::Record, value::Value};
use personio_rs::{
    auth::login,
    personnel::{
        apis::{configuration::Configuration, employees_api::company_employees_get},
        models::EmployeesResponse,
    },
};
use tokio::runtime::Runtime;

mod composite;
mod macros;

const CFG_CLIENT_ID: &str = "client_id";
const CFG_CLIENT_SECRET: &str = "client_secret";
const FLAG_SALARY: &str = "flags.salary";

pub struct Employees {
    token: Option<String>,
    flags: HashMap<String, bool>,
    runtime: Option<Runtime>,
}

impl Employees {
    pub(crate) fn new() -> Self {
        Self {
            token: None,
            flags: HashMap::new(),
            runtime: None,
        }
    }

    fn is_flag_set(&self, flag: &str) -> bool {
        self.flags.get(flag).map_or(false, |&value| value)
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
        macros::add_field!(fields, attr, weekly_working_hours);
        macros::add_field!(fields, attr, hire_date);
        macros::add_field!(fields, attr, contract_end_date);
        macros::add_field!(fields, attr, termination_date);
        macros::add_field!(fields, attr, termination_type);
        macros::add_field!(fields, attr, termination_reason);
        macros::add_field!(fields, attr, probation_period_end);
        macros::add_field!(fields, attr, last_modified_at);
        macros::add_field!(fields, attr, position);
        macros::add_field!(fields, attr, last_working_day);
        macros::add_field!(fields, attr, profile_picture);
        macros::add_field!(fields, attr, dynamic_21827);

        if self.is_flag_set(FLAG_SALARY) {
            macros::add_field!(fields, attr, fix_salary);
            macros::add_field!(fields, attr, fix_salary_interval);
            macros::add_field!(fields, attr, hourly_salary);
        }

        if let Some(hc) = composite::get_holiday_calendar(&attr) {
            add_field(fields, "holiday_calendar", Value::from(hc));
        }

        if let Some(supervisor) = composite::get_supervisor(&attr) {
            add_field(fields, "supervisor", Value::from(supervisor));
        }

        if let Some(subcompany) = composite::get_subcompany(&attr) {
            add_field(fields, "subcompany", Value::from(subcompany));
        }

        if let Some(office) = composite::get_office(&attr) {
            add_field(fields, "office", Value::from(office));
        }

        if let Some(department) = composite::get_department(&attr) {
            add_field(fields, "department", Value::from(department));
        }

        if let Some(cost_centers) = composite::get_cost_centers(&attr) {
            add_field(fields, "cost_centers", Value::from(cost_centers));
        }

        if let Some(work_schedule) = composite::get_work_schedule(&attr) {
            add_field(fields, "work_schedule", Value::from(work_schedule));
        }

        if let Some(absence_entitlement) = composite::get_absence_entitlement(&attr) {
            add_field(fields, "absence_entitlement", Value::from(absence_entitlement));
        }

        if let Some(team) = composite::get_team(&attr) {
            add_field(fields, "team", Value::from(team));
        }

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

            // read flags
            if let Some(salary) = config.get(FLAG_SALARY) {
                if let Ok(salary) = salary.parse::<bool>() {
                    self.flags.insert(String::from(FLAG_SALARY), salary);
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
