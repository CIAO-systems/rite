use config::OllamaConnection;
use import::Importer;
use model::{BoxedError, field::add_field, record::Record, value::Value};
use response::OllamaResponse;
use rig::completion::Prompt;

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

use regex::Regex;
use serde_json::from_str;

fn extract_json_array(input: &str) -> Option<serde_json::Value> {
    // Regex to match JSON block enclosed in triple backticks marked as json
    match Regex::new(r"(?s)```json\s*(.*?)\s*```") {
        Ok(expr) => {
            // Extract the first match
            if let Some(captures) = expr.captures(input) {
                if let Some(json_str) = captures.get(1) {
                    // Parse JSON string into a serde_json::Value
                    let json_value: serde_json::Value = from_str(json_str.as_str()).ok()?;
                    return Some(json_value);
                }
            }
            None
        }
        Err(e) => {
            log::error!("{e}");
            None
        }
    }
}

fn handle_response(
    response: String,
    handler: &mut dyn import::RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "response", Value::String(response));

    handler.handle_record(&mut record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rig::{completion::Prompt, providers::ollama};

    use crate::importers::ollama::response::OllamaResponse;

    use super::extract_json_array;

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
        let input = r#"Here is a raw JSON array containing random German first and last names:

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

        if let Some(json_array) = extract_json_array(input) {
            println!("Extracted JSON:\n{}", json_array);
            assert!(json_array.is_array());
            if let Some(array) = json_array.as_array() {
                assert_eq!(10, array.len());

                let first = array.first().unwrap();
                let first = first.as_object().unwrap();
                assert_eq!("Christoph", first["first"]);
                assert_eq!("Peter", first["last"]);

                let last = array.last().unwrap();
                let last = last.as_object().unwrap();
                assert_eq!("Lukas", last["first"]);
                assert_eq!("Zimmerman", last["last"]);

            }
        } else {
            panic!("No JSON found in the input.");
        }
    }
}
