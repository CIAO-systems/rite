use chrono::NaiveDate;
use ciao_rs::ciao::{
    clients::time_tracking::absences::AbsenceClient, common::Date, time_tracking::absences::Absence,
};
use futures::StreamExt;
use model::import::{Importer, RecordHandler};
use model::{
    field::{add_field, Field},
    record::Record,
    value::Value,
    xml::config::Configuration,
    BoxedError, Initializable,
};

use crate::{config::get_config_time_range, connection::CiaoConnection};

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
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establish connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.absence_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_absences(&self.config, service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }
        Ok(())
    }
}

async fn list_absences(
    config: &Option<Configuration>,
    mut service_client: AbsenceClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), BoxedError> {
    use model::xml::config::get_config_values;
    let mut stream = service_client
        .inner_mut()
        .list(ciao_rs::ciao::time_tracking::absences::ListRequest {
            time_range: get_config_time_range(config, "filter.timeRange"),
            user_ids: get_config_values(config, "filter.userIds"),
            time_type_ids: get_config_values(config, "filter.timeTypeIds"),
        })
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for absence in r.absences {
                    handle_absence(&absence, handler)?;
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn add_date_field(
    fields: &mut Vec<Field>,
    name: &str,
    date: Option<Date>,
) -> Result<(), BoxedError> {
    if let Some(ref date) = date {
        if let Some(date) =
            NaiveDate::from_ymd_opt(date.year, date.month.try_into()?, date.day.try_into()?)
        {
            add_field(
                fields,
                name,
                Value::String(date.format("%Y-%m-%d").to_string()),
            );
        }
    }
    Ok(())
}

fn handle_absence(absence: &Absence, handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();

    // string id = 1;
    add_field(fields, "id", Value::String(absence.id.clone()));

    // ciao.common.Date start_date = 2;
    add_date_field(fields, "startDate", absence.start_date)?;

    // ciao.common.Date end_date = 3;
    add_date_field(fields, "endDate", absence.end_date)?;

    // bool start_half_day = 4;
    add_field(fields, "startHalfDay", Value::Bool(absence.start_half_day));

    // bool end_half_day = 5;
    add_field(fields, "endHalfDay", Value::Bool(absence.end_half_day));

    // string time_type_id = 6;
    add_field(
        fields,
        "timeTypeId",
        Value::String(absence.time_type_id.clone()),
    );

    // string user_id = 7;
    add_field(fields, "userId", Value::String(absence.user_id.clone()));

    // bool deleted = 8;
    add_field(fields, "deleted", Value::Bool(absence.deleted));

    handler.handle_record(&mut record)?;

    Ok(())
}

#[cfg(test)]
mod tests;
