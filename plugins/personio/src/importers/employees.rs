use std::collections::HashMap;

use model::{BoxedError, field::add_field, record::Record, value::Value};
use personio_rs::personnel::models::EmployeesResponse;

use crate::macros;

use super::configuration::GeneralConfiguration;

mod composite;
mod importer;
mod initializable;

const FLAG_SALARY: &str = "flags.salary";

pub struct Filter {
    email: Option<String>,
    updated_since: Option<String>,
    attributes: Option<Vec<String>>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            email: None,
            updated_since: None,
            attributes: None,
        }
    }

    fn set_attributes(&mut self, attributes: String) {
        self.attributes = Some(attributes.split(',').map(|s| s.to_string()).collect());
    }
}

pub struct Employees {
    general: GeneralConfiguration,
    flags: HashMap<String, bool>,
    limit: Option<i32>,
    filter: Filter,
}

impl Employees {
    pub(crate) fn new() -> Self {
        Self {
            general: GeneralConfiguration::new(),
            flags: HashMap::new(),
            limit: None,
            filter: Filter::new(),
        }
    }

    fn is_flag_set(&self, flag: &str) -> bool {
        self.flags.get(flag).map_or(false, |&value| value)
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
            add_field(
                fields,
                "absence_entitlement",
                Value::from(absence_entitlement),
            );
        }

        if let Some(team) = composite::get_team(&attr) {
            add_field(fields, "team", Value::from(team));
        }

        // Add all "unknown" properties. Use "universal_id" as field name, when available
        for (key, value) in &attr.additional_properties {
            let id = value["universal_id"]
                .as_str()
                .unwrap_or_else(|| key)
                .to_string();

            add_field(fields, &id, Value::from(value["value"].clone()));
        }

        Ok(record)
    }
}
