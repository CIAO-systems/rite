use ciao_rs::{
    ciao::{
        interceptor::APIKeyClientInterceptor,
        time_tracking::project::{GetRequest, ListRequest},
        ClientManager,
    },
    interceptors,
};
use futures::StreamExt;
use import::{handlers::CollectingRecordHandler, RecordHandler};
use model::{
    field::Field,
    record::Record,
    xml::config::{ConfigItem, Configuration},
    BoxedError,
};
use tokio::runtime::Runtime;

use super::CiaoConnection;

const REMOTE_BACKEND_URL: &str = "https://backend-api.ciao.software:443";
const REMOTE_API_KEY: &str = "c776ba7c-e1ac-43f6-8c25-91fc993afad0";
const REMOTE_PROJECT_ID: &str = "2fae3e46-f4d2-4300-9e36-5159f9de9c9f";

fn get_remote_config() -> Option<Configuration> {
    Some(Configuration {
        xml: None,
        config: Some(vec![
            ConfigItem {
                key: String::from("url"),
                value: String::from(REMOTE_BACKEND_URL),
            },
            ConfigItem {
                key: String::from("api-key"),
                value: String::from(REMOTE_API_KEY),
            },
        ]),
    })
}

#[test]
#[ignore = "for manual testing"]
fn manual_connection() -> Result<(), BoxedError> {
    let config = get_remote_config();

    let connection = CiaoConnection::connect(&config)?;
    if let Some(client) = connection.client {
        let mut pc = client.project_client;
        if let Some(runtime) = connection.runtime {
            runtime.block_on(async {
                let r = pc
                    .inner_mut()
                    .get(GetRequest {
                        id: REMOTE_PROJECT_ID.to_string(),
                    })
                    .await;
                println!("{:?}", r);
            });
        }
    }

    Ok(())
}

#[test]
#[ignore = "for manual testing"]
fn manual_connection_config() -> Result<(), BoxedError> {
    let rt = Runtime::new()?;
    let result: Result<(), BoxedError> = rt.block_on(async {
        let mut manager = ClientManager::new(
            "http://localhost:50051",
            interceptors!(APIKeyClientInterceptor::new(
                "top-secret-api-key".to_string()
            )),
        )
        .await?;
        let values = manager.configuration_client.get_values(None).await?;
        for entry in &values.entries {
            println!("{} = {}", entry.0, entry.1);
        }

        assert_eq!(
            values.entries.get("api_key"),
            Some(&String::from("abcdef123456"))
        );
        assert_eq!(
            values.entries.get("max_connections"),
            Some(&String::from("100"))
        );
        assert_eq!(
            values.entries.get("database_url"),
            Some(&String::from("postgresql://localhost:5432/mydb"))
        );
        assert_eq!(
            values.entries.get("debug_mode"),
            Some(&String::from("true"))
        );
        assert_eq!(values.entries.get("cache_ttl"), Some(&String::from("3600")));
        Ok(())
    });

    assert!(result.is_ok());
    Ok(())
}

#[test]
#[ignore = "for manual testing"]
fn manual_connection_projects() -> Result<(), BoxedError> {
    let rt = Runtime::new()?;
    let result: Result<(), BoxedError> = rt.block_on(async {
        let mut manager = ClientManager::new(
            "https://backend-api.ciao.software:443",
            interceptors!(APIKeyClientInterceptor::new(
                "c776ba7c-e1ac-43f6-8c25-91fc993afad0".to_string(),
            )),
        )
        .await?;

        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        let request = ListRequest { active_at: None };
        let result: Result<(), BoxedError> =
            match manager.project_client.inner_mut().list(request).await {
                Ok(response) => {
                    //
                    let mut stream = response.into_inner();
                    while let Some(response) = stream.next().await {
                        match response {
                            Ok(response) => {
                                //
                                for project in response.projects {
                                    let s = format!("{:#?}", project);
                                    let mut record = Record::new();
                                    record
                                        .fields_as_mut()
                                        .push(Field::new_string("debug".to_string(), s.clone()));
                                    println!("{s}");

                                    if let Err(e) = handler.handle_record(&mut record) {
                                        log::error!("Error while handling record: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("{}", e);
                            }
                        }
                    }

                    Ok(())
                }
                Err(e) => Err(e.into()),
            };

        if let Err(e) = result {
            panic!("{e}");
        }

        for r in records {
            println!("{:?}", r);
        }

        Ok(())
    });

    if let Err(e) = result {
        panic!("{}", e);
    }

    Ok(())
}
