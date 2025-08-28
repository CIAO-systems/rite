use filter::AbsencesFilter;
use model::import::RecordHandler;
use model::{BoxedError, field::add_field, record::Record, value::Value};
use personio_rs::personnel::models::{AbsenceTimeOffType, ShortEmployee};

use crate::macros::{add_field_boxed, add_field_direct};

use super::configuration::GeneralConfiguration;

mod filter;
mod importer;
mod initializable;

pub struct Absences {
    general: GeneralConfiguration,
    filter: AbsencesFilter,
}
impl Absences {
    pub(crate) fn new() -> Self {
        Self {
            general: GeneralConfiguration::new(),
            filter: AbsencesFilter::new(),
        }
    }

    fn handle_response(
        &self,
        handler: &mut dyn RecordHandler,
        page: personio_rs::personnel::models::AbsencePeriodsResponse,
    ) -> Result<(), BoxedError> {
        if page.success {
            for absence in page.data {
                let mut record = Record::new();
                let attributes = &absence.attributes;
                add_field_direct!(record, attributes, id);
                add_field_direct!(record, attributes, status);
                add_field_direct!(record, attributes, created_at);
                add_field_direct!(record, attributes, created_by);
                add_field_direct!(record, attributes, updated_at);
                add_field_direct!(record, attributes, days_count);
                add_field_direct!(record, attributes, start_date);
                add_field_direct!(record, attributes, end_date);
                add_field_direct!(record, attributes, half_day_end);
                add_field_direct!(record, attributes, half_day_start);

                add_employee(&mut record, &attributes.employee);
                add_time_off_type(&mut record, &attributes.time_off_type);

                for (key, value) in &absence.additional_properties {
                    add_field(record.fields_as_mut(), &key, Value::from(value.clone()));
                }

                handler.handle_record(&mut record)?;
            }
        }
        Ok(())
    }
}

fn add_time_off_type(record: &mut Record, time_off_type: &Option<Box<AbsenceTimeOffType>>) {
    if let Some(time_off_type) = time_off_type {
        if let Some(attributes) = &time_off_type.attributes {
            let mut time_off_type_record = Record::new();
            add_field_direct!(time_off_type_record, attributes, id);
            add_field_direct!(time_off_type_record, attributes, category);
            add_field_direct!(time_off_type_record, attributes, name);
            add_field(
                record.fields_as_mut(),
                "time_off_type",
                time_off_type_record.into(),
            );
        }
    }
}

fn add_employee(record: &mut Record, employee: &Option<Box<ShortEmployee>>) {
    if let Some(employee) = employee {
        let mut employee_record = Record::new();
        if let Some(attributes) = &employee.attributes {
            add_field_boxed!(employee_record, attributes, id);
            add_field_boxed!(employee_record, attributes, first_name);
            add_field_boxed!(employee_record, attributes, last_name);
            add_field_boxed!(employee_record, attributes, email);

            add_field(record.fields_as_mut(), "employee", employee_record.into());
        }
    }
}
