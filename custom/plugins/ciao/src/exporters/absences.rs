use chrono::{DateTime, Datelike, NaiveDate, Utc};
use ciao_rs::ciao::{
    clients::time_tracking::absences::AbsenceClient,
    common::Date,
    time_tracking::absences::{Absence, CreateRequest},
};
use export::Exporter;
use model::{value::Value, BoxedError, Initializable};
use uuid::Uuid;

use crate::connection::CiaoConnection;

pub struct Absences {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl Absences {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for Absences {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        // 1. Establish connection to gRPC server
        self.connection = Some(CiaoConnection::connect(&self.config)?);

        Ok(())
    }
}

impl Exporter for Absences {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.absence_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_absence(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }

        Ok(())
    }
}

async fn create_absence(
    service_client: &mut AbsenceClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let absence = Some(absence_from_record(record)?);

    service_client
        .inner_mut()
        .create(CreateRequest { absence })
        .await?;
    Ok(())
}

fn absence_from_record(record: &model::record::Record) -> Result<Absence, BoxedError> {
    let id = Uuid::new_v4().to_string();
    let start_date = Some(get_date(record, "startDate")?);
    let end_date = Some(get_date(record, "endDate")?);
    let start_half_day = get_bool(record, "startHalfDay", false);
    let end_half_day = get_bool(record, "endHalfDay", false);
    let time_type_id = get_mandatory_string(record, "timeTypeId")?;
    let user_id = get_mandatory_string(record, "userId")?;
    let deleted = false;

    Ok(Absence {
        id,
        start_date,
        end_date,
        start_half_day,
        end_half_day,
        time_type_id,
        user_id,
        deleted,
    })
}

fn get_mandatory_string(
    record: &model::record::Record,
    field_name: &str,
) -> Result<String, BoxedError> {
    Ok(record
        .field_by_name(field_name)
        .map(|field| field.value().to_string())
        .ok_or_else(|| BoxedError::from(format!("Mandatory field '{field_name}' not found")))?)
}

fn get_bool(record: &model::record::Record, field_name: &str, default: bool) -> bool {
    if let Some(field) = record.field_by_name(field_name) {
        if let Value::Bool(value) = field.value() {
            return value;
        }
    }

    default
}

fn get_date(record: &model::record::Record, field_name: &str) -> Result<Date, BoxedError> {
    let field = record
        .field_by_name(field_name)
        .ok_or_else(|| BoxedError::from(format!("Mandatory field '{field_name}' not found")))?;

    if let Value::String(field_value) = field.value() {
        match NaiveDate::parse_from_str(&field_value, "%Y-%m-%d") {
            Ok(date) => {
                return Ok(Date {
                    year: date.year(),
                    month: date.month0() as i32 + 1,
                    day: date.day0() as i32 + 1,
                })
            }
            Err(_) => {
                let date_time = field_value
                    .parse::<DateTime<Utc>>()
                    .map_err(|e| format!("Field '{field_name}' is invalid: {e}"))?;
                return Ok(Date {
                    year: date_time.year(),
                    month: date_time.month0() as i32 + 1,
                    day: date_time.day0() as i32 + 1,
                });
            }
        }
    }

    Err(format!("Mandatory field '{field_name}' is not a YYYY-MM-DD string").into())
}

#[cfg(test)]
mod tests {
    use model::{field::add_field, record::Record, value::Value};

    use super::*;

    #[test]
    fn test_get_mandatory_string() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "field",
            Value::String("value".to_string()),
        );
        let result = get_mandatory_string(&record, "field");
        assert!(result.is_ok());
        assert_eq!("value", result.unwrap());
    }

    #[test]
    fn test_get_mandatory_string_missing() {
        let record = Record::new();
        let result = get_mandatory_string(&record, "field");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!("Mandatory field 'field' not found", e);
    }

    #[test]
    fn test_get_bool() {
        let mut record = Record::new();
        add_field(record.fields_as_mut(), "bool", Value::Bool(true));
        let result = get_bool(&record, "bool", false);
        assert!(result);
    }

    #[test]
    fn test_get_bool_default() {
        let record = Record::new();
        let result = get_bool(&record, "bool", false);
        assert!(!result);
        let result = get_bool(&record, "bool", true);
        assert!(result);
    }

    #[test]
    fn test_get_date() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "date",
            Value::String("2025-01-01".to_string()),
        );

        let result = get_date(&record, "date");
        assert!(result.is_ok());
        let value = result.unwrap();

        assert_eq!(
            value,
            Date {
                year: 2025,
                month: 1,
                day: 1
            }
        );
    }

    #[test]
    fn test_get_date_missing() {
        let record = Record::new();
        let result = get_date(&record, "date");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!("Mandatory field 'date' not found", e);
    }

    #[test]
    fn test_get_date_invalid() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "date",
            Value::String("This is not a date".to_string()),
        );
        let result = get_date(&record, "date");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!(
            "Field 'date' is invalid: input contains invalid characters",
            e
        );
    }

    #[test]
    fn test_get_date_with_time() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "date",
            Value::String("2025-01-01T23:00:00Z".to_string()),
        );
        let result = get_date(&record, "date");
        assert!(result.is_ok(), "Result should be ok, but is err");
        assert_eq!(
            result.unwrap(),
            Date {
                year: 2025,
                month: 1,
                day: 1
            }
        );
    }

    #[test]
    fn test_get_date_no_string() {
        let mut record = Record::new();
        add_field(record.fields_as_mut(), "date", Value::I32(4273));

        let result = get_date(&record, "date");
        assert!(result.is_err());
        let e = format!("{}", result.unwrap_err());
        assert_eq!("Mandatory field 'date' is not a YYYY-MM-DD string", e);
    }
}
