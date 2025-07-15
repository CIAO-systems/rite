use chrono::{DateTime, Datelike, Local, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{field::add_field, record::Record, value::Value, BoxedError, Initializable};
use prost_types::Timestamp;

use crate::{
    com::atoss::atc::protobuf::AbsencesRequest,
    connection::ATCConnection,
    importers::common::{date_to_protobuf, parse_period, timestamp_to_string},
};

const CFG_FILTER_EMPLOYEES: &str = "filter.employees";
const CFG_FILTER_PERIOD: &str = "filter.period";
const CFG_FILTER_ACCOUNTS: &str = "filter.accounts";

pub struct Absences {
    config: Option<model::xml::config::Configuration>,
}

impl Absences {
    pub fn new() -> Self {
        Absences { config: None }
    }
}

impl Initializable for Absences {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Absences {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establish connection to gRPC server
        let connection = ATCConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.absences_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    if let Some(ref config) = self.config {
                        call_get_absences(config, service_client, handler).await?;
                    }

                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

/// Calls the AbsencesService::getSingleDayAbsences service
async fn call_get_absences(
    config: &model::xml::config::Configuration,
    mut service_client: crate::connection::clients::AbsencesClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = create_request(config)?;

    let mut stream = service_client
        .inner_mut()
        .get_single_day_absences(request)
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(absence) => {
                handle_absence(handler, absence)?;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn handle_absence(
    handler: &mut dyn RecordHandler,
    absence: crate::com::atoss::atc::protobuf::Absence,
) -> Result<(), BoxedError> {
    // ignore records that are not planVersion=1
    if absence.plan_version != 1 {
        return Ok(());
    }

    let mut record = Record::new();
    let fields = record.fields_as_mut();

    add_field(fields, "accountId", Value::I32(absence.account_id));
    add_field(fields, "application", Value::String(absence.application));
    add_timestamp_field(fields, "date", absence.date);
    add_field(fields, "description", Value::String(absence.description));
    add_field(fields, "displayColor", Value::I32(absence.display_color));
    add_field(fields, "displayToken", Value::String(absence.display_token));
    add_field(fields, "employeeId", Value::String(absence.employee_id));
    add_timestamp_field(fields, "endDate", absence.end_date);
    add_field(fields, "planVersion", Value::I32(absence.plan_version));
    add_field(fields, "remark", Value::String(absence.remark));
    add_timestamp_field(fields, "startDate", absence.start_date);
    add_field(fields, "state", Value::I32(absence.state));
    add_field(fields, "substitute", Value::String(absence.substitute));
    add_field(fields, "textColor", Value::I32(absence.text_color));
    add_timestamp_field(fields, "time", absence.time);
    add_field(fields, "weightEnd", Value::F64(absence.weight_end));
    add_field(fields, "weightStart", Value::F64(absence.weight_start));

    handler.handle_record(&mut record)?;
    Ok(())
}

fn add_timestamp_field(
    fields: &mut Vec<model::field::Field>,
    field_name: &str,
    value: Option<Timestamp>,
) {
    if let Some(date) = value {
        if let Ok(local_ts) = utc_to_atc(date) {
            add_field(
                fields,
                field_name,
                Value::String(timestamp_to_string(local_ts)),
            );
        }
    }
}

/// Convert UTC timestamp to German time
fn utc_to_atc(date: Timestamp) -> Result<Timestamp, BoxedError> {
    // ATC stores timestamps in local time (we assume German), but returns them
    // as UTC timestamps.
    if let Some(utc_datetime) = DateTime::<Utc>::from_timestamp(date.seconds, date.nanos as u32) {
        let berlin_time = utc_datetime.with_timezone(&Berlin);
        let naive_datetime = berlin_time.naive_local();
        let utc_datetime = Utc.from_utc_datetime(&naive_datetime);
        return Ok(Timestamp {
            seconds: utc_datetime.timestamp(),
            nanos: utc_datetime.timestamp_subsec_nanos() as i32,
        });
    }

    Err("utc_to_atc failed".into())
}

fn create_request(
    config: &model::xml::config::Configuration,
) -> Result<AbsencesRequest, BoxedError> {
    let (start_date, end_date) = get_start_and_end_date(config)?;
    let request = AbsencesRequest {
        employee_ids: config.get_list(CFG_FILTER_EMPLOYEES).unwrap_or(Vec::new()),
        start_date,
        end_date,
        account_ids: config.get_list(CFG_FILTER_ACCOUNTS).unwrap_or(Vec::new()),
        plan_version: -1,
        options: None,
    };
    Ok(request)
}

fn get_start_and_end_date(
    config: &model::xml::config::Configuration,
) -> Result<(Option<Timestamp>, Option<Timestamp>), BoxedError> {
    let mut start_date: Option<Timestamp> = None;
    let mut end_date: Option<Timestamp> = None;
    if let Some(period_str) = config.get(CFG_FILTER_PERIOD) {
        let (start, end) = parse_period(&period_str);
        if let Some(start) = start {
            start_date = Some(date_to_protobuf(&start)?);
        } else {
            // if start date is missing take current date
            start_date = Some(date_to_protobuf(&Local::now().date_naive())?);
        }

        if let Some(end) = end {
            end_date = Some(date_to_protobuf(&end)?);
        } else {
            // If end date is missing, take a one year period
            let today = Local::now().date_naive();
            if let Some(date) = today.with_year(today.year() + 1) {
                end_date = Some(date_to_protobuf(&date)?);
            }
        }
    }
    Ok((start_date, end_date))
}

#[cfg(test)]
mod tests;
