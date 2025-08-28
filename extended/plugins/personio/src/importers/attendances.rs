use filter::AttendancesFilter;
use model::import::RecordHandler;
use model::{BoxedError, field::add_field, record::Record, value::Value};

use crate::macros;

use super::configuration::GeneralConfiguration;

mod importer;
mod initializable;
mod filter;

pub struct Attendances {
    general: GeneralConfiguration,
    filter: AttendancesFilter,
}

impl Attendances {
    pub fn new() -> Self {
        Self {
            general: GeneralConfiguration::new(),
            filter: AttendancesFilter::new(),
        }
    }

    fn handle_attendance_response(
        &self,
        handler: &mut dyn RecordHandler,
        page: personio_rs::personnel::models::AttendancePeriodsResponse,
    ) -> Result<(), BoxedError> {
        if page.success {
            for attendance in page.data {
                let mut record = Record::new();

                macros::add_field_none!(record, attendance, id);

                let attributes = &attendance.attributes;
                macros::add_field_direct!(record, attributes, employee);
                macros::add_field_direct!(record, attributes, date);
                macros::add_field_direct!(record, attributes, start_time);
                macros::add_field_option!(record, attributes, end_time);
                macros::add_field_direct!(record, attributes, r#break);
                macros::add_field_direct!(record, attributes, comment);
                macros::add_field_direct!(record, attributes, is_holiday);
                macros::add_field_direct!(record, attributes, is_on_time_off);

                add_status(&mut record, attributes);
                add_project(&mut record, attributes);

                for (key, value) in &attendance.additional_properties {
                    add_field(record.fields_as_mut(), &key, Value::from(value.clone()));
                }

                handler.handle_record(&mut record)?;
            }
        }
        Ok(())
    }
}

fn add_project(record: &mut Record, attributes: &Box<personio_rs::personnel::models::Attendance>) {
    if let Some(ref project) = attributes.project {
        if let Some(project) = project {
            let mut project_record = Record::new();
            macros::add_field_direct!(project_record, project, id);
            if let Some(ref attributes) = project.attributes {
                macros::add_field_direct!(project_record, attributes, active);
                macros::add_field_direct!(project_record, attributes, name);
            }
            add_field(record.fields_as_mut(), "project", project_record.into());
        }
    }
}

fn add_status(record: &mut Record, attributes: &Box<personio_rs::personnel::models::Attendance>) {
    if let Some(status) = attributes.status {
        add_field(
            record.fields_as_mut(),
            "status",
            format!("{:?}", status).into(),
        );
    }
}
