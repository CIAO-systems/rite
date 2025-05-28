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
                let response = connection.agent.prompt(&prompt).await?;
                let response = OllamaResponse::new(&response)?;
                if let Some(thinking) = response.thinking {
                    log::debug!("LLM was thinking about this:\n{thinking}\n\n")
                }

                if let Some(response) = response.response {
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
    for object in objects {
        println!("{:?}", object);
    }
    let mut record = Record::new();

    add_field(record.fields_as_mut(), "response", Value::String(response));

    handler.handle_record(&mut record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rig::{completion::Prompt, providers::ollama};
    use serde_json::Value;

    use crate::importers::ollama::response::OllamaResponse;

    use super::extract_json_structures;

    const TEXT_WITH_JSON1: &str = r#"Here is a raw JSON array containing random German first and last names:

```json
[
  {
    "first": "Christoph",
    "last": "Peter"
  },
  {
    "first": "Maria",
    "last": "Anna"
  },
  {
    "first": "Lena",
    "last": "Haus"
  },
  {
    "first": "Hans",
    "last": "Müller"
  },
  {
    "first": "Julia",
    "last": "Braun"
  },
  {
    "first": "Alexander",
    "last": "Witt"
  },
  {
    "first": "Sophie",
    "last": "Keller"
  },
  {
    "first": "Felix",
    "last": "Hahn"
  },
  {
    "first": "Eva",
    "last": "Richter"
  },
  {
    "first": "Lukas",
    "last": "Zimmerman"
  }
]
```"#;
    const TEXT_WITH_JSON2: &str =
        r#"Some text {"key": "value"} and some more text [1, 2, 3] and more text"#;

    #[tokio::test]
    #[ignore = "for manual testing, because it is too slow and it needs an Ollama running"]
    async fn test_ollama() -> Result<(), Box<dyn std::error::Error>> {
        // To start a local Ollama, you can use the script
        // [ollama.sh](https://github.com/CIAO-systems/assets-scripts/blob/main/internal/ollama.sh)

        // Create a new Ollama client (defaults to http://localhost:11434)
        let client = ollama::Client::from_url("http://localhost:11434");

        // Create an agent
        let agent = client
            .agent("deepseek-r1:7b")
            .preamble("Always answer in form of a raw (no markdown) JSON list of records with key/value pairs. Do not add any notes!")
            .build();

        let response = agent
            .prompt("List ten European cities with the number of people living there.")
            .await?;
        let response = OllamaResponse::new(&response)?;
        if let Some(thinking) = response.thinking {
            println!("LLM was thinking about this:\n{thinking}\n\n")
        }
        if let Some(response) = response.response {
            println!("LLM response was:\n{response}");
        }

        Ok(())
    }

    #[test]
    fn test_extract_json_array() {
        let values = extract_json_structures(TEXT_WITH_JSON1);
        assert_eq!(1, values.len());
        assert!(values[0].is_array());

        if let Some(array) = values[0].as_array() {
            println!("Extracted JSON:\n{:?}", array);
            assert_eq!(10, array.len());

            let first = array.first().unwrap();
            let first = first.as_object().unwrap();
            assert_eq!("Christoph", first["first"]);
            assert_eq!("Peter", first["last"]);

            let last = array.last().unwrap();
            let last = last.as_object().unwrap();
            assert_eq!("Lukas", last["first"]);
            assert_eq!("Zimmerman", last["last"]);
        } else {
            panic!("No JSON found in the input.");
        }
    }

    #[test]
    fn test_extract_json_structures() {
        let objects = extract_json_structures(TEXT_WITH_JSON1);
        assert_eq!(1, objects.len());

        let objects = extract_json_structures(TEXT_WITH_JSON2);
        assert_eq!(2, objects.len());
        
        // Element 0 is a object
        let object = objects[0].as_object().unwrap();
        let keys: Vec<String> = object.keys().cloned().collect();
        assert_eq!("key", keys[0]);
        let values: Vec<Value> = object.values().cloned().collect();
        assert_eq!("value", values[0].as_str().unwrap());

        // Element 1 is an array
        let array = objects[1].as_array().unwrap();
        assert_eq!(3, array.len());
        for (i, v) in array.iter().enumerate() {
            assert_eq!(i as u64, v.as_u64().unwrap() - 1, "Index should be value - 1");
        }

    }
}
