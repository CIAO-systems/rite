use config::OllamaConnection;
use import::Importer;
use model::{BoxedError, field::add_field, record::Record, value::Value};
use response::OllamaResponse;
use rig::completion::Prompt;
use serde_json::Value as JSONValue;

pub mod config;
pub mod response;

#[derive(Default)]
pub struct OllamaImporter {
    prompt: Option<String>,
    connection: Option<OllamaConnection>,
}

impl Importer for OllamaImporter {
    fn read(&mut self, handler: &mut dyn import::RecordHandler) -> Result<(), BoxedError> {
        let prompt = self.prompt.clone().ok_or("No 'prompt' configured")?;

        if let Some(ref connection) = self.connection {
            // Use the connection tokio runtime to execute the prompt
            let result: Result<(), BoxedError> = connection.runtime.block_on(async {
                // Logging does not work, see https://ciao-systems.youtrack.cloud/issue/RIT-51/Plugins-cant-log-in-main-log-framework
                log::debug!("Sending prompt:\n{prompt}\n\n");

                // for debugging purposes:
                // let response = std::fs::read_to_string("base/data/test/llm/example-response.json")?;
                let response = connection.agent.prompt(&prompt).await?;

                log::debug!("Agent response:\n{response}");

                let response = OllamaResponse::new(&response)?;
                if let Some(thinking) = response.thinking {
                    log::debug!("Agent thinking:\n{thinking}")
                }

                if let Some(response) = response.response {
                    log::debug!("Agent answer:\n{response}");
                    handle_response(response, handler)?;
                }

                Ok(())
            });
            return result;
        }
        Ok(())
    }
}

fn extract_json_structures(input: &str) -> Vec<JSONValue> {
    let mut json_structures = Vec::new();
    let mut stack = Vec::new();
    let mut start_index = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '{' | '[' => {
                if stack.is_empty() {
                    start_index = i;
                }
                stack.push(c);
            }
            '}' | ']' => {
                if let Some(_) = stack.pop() {
                    if stack.is_empty() {
                        let json_str = &input[start_index..=i + 1];
                        match serde_json::from_str::<JSONValue>(json_str) {
                            Ok(value) => {
                                json_structures.push(value);
                            }
                            Err(e) => {
                                log::error!("{e}");
                                eprintln!("{e}");
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    json_structures
}

fn handle_response(
    response: String,
    handler: &mut dyn import::RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let objects = extract_json_structures(&response);
    log::debug!("Extracted {} JSON structures", objects.len());

    for object in objects {
        match object {
            JSONValue::Array(values) => {
                for value in values {
                    if let Some(map) = value.as_object() {
                        let mut record = extract_record(map);
                        handler.handle_record(&mut record)?;
                    }
                }
            }
            JSONValue::Object(map) => {
                let mut record = extract_record(&map);
                handler.handle_record(&mut record)?;
            }
            _ => (),
        }
    }

    Ok(())
}

fn extract_record(map: &serde_json::Map<String, serde_json::Value>) -> Record {
    let mut record = Record::new();

    for (key, value) in map.iter() {
        log::debug!("{}={}", key, value);
        add_field(record.fields_as_mut(), key, Value::from(value.clone()));
    }

    record
}

#[cfg(test)]
mod tests;
