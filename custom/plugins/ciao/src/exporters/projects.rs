use ciao_rs::ciao::time_tracking::project::{CreateRequest, Project};
use export::Exporter;
use model::Initializable;

use crate::{connection::CiaoConnection, model::get_timestamp};

pub struct Projects {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl Projects {
    pub fn new() -> Self {
        Projects {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for Projects {
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

impl Exporter for Projects {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.project_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_project(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }
        Ok(())
    }
}

async fn create_project(
    service_client: &mut ciao_rs::ciao::clients::time_tracking::projects::ProjectClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let request = CreateRequest {
        project: Some(project_from_record(record)),
    };

    let response = service_client
        .inner_mut()
        .create(request)
        .await?
        .into_inner();
    log::info!("Project {:?} created", response);
    Ok(())
}

fn project_from_record(record: &model::record::Record) -> Project {
    Project {
        id: record
            .field_by_name("id")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        external_id: record
            .field_by_name("externalId")
            .map(|v| v.value().to_string()),
        name: record
            .field_by_name("name")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        start_date: match get_timestamp(record, "startDate") {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("{e}");
                None
            }
        },
        end_date: match get_timestamp(record, "endDate") {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("{e}");
                None
            }
        },
        closed_date: match get_timestamp(record, "closedDate") {
            Ok(v) => Some(v),
            Err(e) => {
                log::error!("{e}");
                None
            }
        },
        parent_id: record
            .field_by_name("parentId")
            .map(|v| Some(v.value().to_string()))
            .unwrap_or(None),
    }
}

#[cfg(test)]
mod tests {
    use super::{project_from_record, Projects};
    use crate::model::add_timestamp_parse;
    use export::Exporter;
    use model::{
        field::add_field, record::Record, value::Value, xml::config::Configuration, BoxedError,
        Initializable,
    };

    fn create_test_record() -> Result<Record, BoxedError> {
        let mut record = Record::new();
        let fields = record.fields_as_mut();
        add_field(fields, "id", Value::String("project-id".to_string()));
        add_field(
            fields,
            "externalId",
            Value::String("external-id".to_string()),
        );
        add_field(fields, "name", Value::String("project-name".to_string()));
        add_timestamp_parse(fields, "startDate", "2025-02-01 08:00", "%Y-%m-%d %H:%M")?;
        add_timestamp_parse(fields, "endDate", "2025-02-28 23:00", "%Y-%m-%d %H:%M")?;
        add_timestamp_parse(fields, "closedDate", "2025-03-01 00:00", "%Y-%m-%d %H:%M")?;
        add_field(fields, "parentId", Value::String("parent-id".to_string()));
        Ok(record)
    }

    #[test]
    fn test_project_from_record() -> Result<(), BoxedError> {
        let record = create_test_record()?;
        let project = project_from_record(&record);
        assert_eq!(project.id, "project-id");
        assert_eq!(project.external_id, Some("external-id".to_string()));
        assert_eq!(project.name, "project-name");
        assert_eq!(project.parent_id, Some("parent-id".to_string()));
        assert_date(project.start_date, 1738396800, None);
        assert_date(project.end_date, 1740783600, None);
        assert_date(project.closed_date, 1740787200, None);

        Ok(())
    }

    fn assert_date(v: Option<ciao_rs::ciao::common::Timestamp>, seconds: i64, tz: Option<&str>) {
        assert!(v.is_some());
        let v = v.unwrap();
        assert!(v.time_utc.is_some());
        if let Some(tz) = tz {
            assert_eq!(v.time_zone, tz);
        }
        let v = v.time_utc.unwrap();
        assert_eq!(v.seconds, seconds);
    }

    #[test]
    #[ignore = "for manual testing"]
    fn test_project_exporter() -> Result<(), BoxedError> {
        let mut exporter = Projects::new();
        let mut config = Configuration::new();
        config.insert_str("url", "http://localhost:50051");
        config.insert_str("api-key", "top-secret-api-key");

        exporter.init(Some(config))?;

        let record = create_test_record()?;

        exporter.write(&record)?;

        Ok(())
    }
}
