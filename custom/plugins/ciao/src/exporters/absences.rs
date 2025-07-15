use ciao_rs::ciao::{
    clients::time_tracking::absences::AbsenceClient,
    time_tracking::absences::{Absence, CreateRequest},
};
use export::Exporter;
use model::{BoxedError, Initializable};
use uuid::Uuid;

use crate::{connection::CiaoConnection, model::{get_bool, get_date, get_mandatory_string}};

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
