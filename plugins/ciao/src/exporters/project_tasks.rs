use ciao_rs::ciao::time_tracking::project::task::{CreateRequest, ProjectTask};
use export::Exporter;
use model::Initializable;

use crate::connection::CiaoConnection;

pub struct ProjectTasks {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl ProjectTasks {
    pub fn new() -> Self {
        ProjectTasks {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for ProjectTasks {
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

impl Exporter for ProjectTasks {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.project_task_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_project_task(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }
        Ok(())
    }
}

async fn create_project_task(
    service_client: &mut ciao_rs::ciao::clients::time_tracking::project_tasks::ProjectTaskClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let request = CreateRequest {
        task: Some(task_from_record(record)),
    };
    let response = service_client
        .inner_mut()
        .create(request)
        .await?
        .into_inner();
    log::info!("Project task {:?} created", response);

    Ok(())
}

fn task_from_record(
    record: &model::record::Record,
) -> ciao_rs::ciao::time_tracking::project::task::ProjectTask {
    ProjectTask {
        id: record
            .field_by_name("id")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        project_id: record
            .field_by_name("projectId")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
        name: record
            .field_by_name("name")
            .map(|v| v.value().to_string())
            .unwrap_or("".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use export::Exporter;
    use model::{
        field::add_field, record::Record, value::Value, xml::config::Configuration, BoxedError,
        Initializable,
    };

    use crate::exporters::project_tasks::task_from_record;

    use super::ProjectTasks;

    fn create_test_record() -> Result<Record, BoxedError> {
        let mut record = Record::new();
        let fields = record.fields_as_mut();
        add_field(fields, "id", Value::String("task-id".to_string()));
        add_field(fields, "projectId", Value::String("project-id".to_string()));
        add_field(fields, "name", Value::String("task-name".to_string()));
        Ok(record)
    }

    #[test]
    fn test_project_from_record() -> Result<(), BoxedError> {
        let record = create_test_record()?;
        let task = task_from_record(&record);
        assert_eq!(task.id, "task-id");
        assert_eq!(task.project_id, "project-id".to_string());
        assert_eq!(task.name, "task-name");
        Ok(())
    }

    #[test]
    #[ignore = "for manual testing"]
    fn test_project_task_exporter() -> Result<(), BoxedError> {
        let mut exporter = ProjectTasks::new();
        let mut config = Configuration::new();
        config.insert_str("url", "http://localhost:50051");
        config.insert_str("api-key", "top-secret-api-key");

        exporter.init(Some(config))?;

        let record = create_test_record()?;

        exporter.write(&record)?;

        Ok(())
    }
}
