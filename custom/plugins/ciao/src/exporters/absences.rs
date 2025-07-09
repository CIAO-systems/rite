use chrono::{Datelike, NaiveDate};
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
        // 1. Establich connection to gRPC server
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
    let deleted = get_bool(record, "deleted", false);

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
        .field_by_name("timeTypeId")
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
        let naive_date = NaiveDate::parse_from_str(&field_value, "%Y-%m-%d")?;

        return Ok(Date {
            year: naive_date.year(),
            month: naive_date.month0() as i32 + 1,
            day: naive_date.day0() as i32 + 1,
        });
    }

    Err(format!("Mandatory field '{field_name}' is not a string").into())
}
