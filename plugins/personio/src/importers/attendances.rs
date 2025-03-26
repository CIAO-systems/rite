use import::RecordHandler;
use model::{BoxedError, field::add_field, record::Record, value::Value};

use crate::macros;

use super::configuration::GeneralConfiguration;

mod importer;
mod initializable;

const CFG_FILTER_START_DATE: &str = "filter.start_date";
const CFG_FILTER_END_DATE: &str = "filter.end_date";

pub struct AttendancesFilter {
    start_date: String,
    end_date: String,
}

impl AttendancesFilter {
    fn new() -> Self {
        Self {
            start_date: "".to_string(),
            end_date: "".to_string(),
        }
    }

    fn load(config: &model::xml::config::Configuration) -> Result<Self, BoxedError> {
        let start_date = config.get_result(CFG_FILTER_START_DATE)?;
        let end_date = config.get_result(CFG_FILTER_END_DATE)?;

        Ok(Self {
            start_date,
            end_date,
        })
    }
}

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

                // TODO status
                // TODO: project

                for (key, value) in &attendance.additional_properties {
                    add_field(record.fields_as_mut(), &key, Value::from(value.clone()));
                }

                //println!("{:#?}", attendance);

                handler.handle_record(&mut record)?;
            }
        }
        Ok(())
    }
}
