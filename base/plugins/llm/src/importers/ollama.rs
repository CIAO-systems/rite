use config::OllamaConnection;
use import::Importer;
use model::{field::add_field, record::Record, value::Value};

pub mod config;
pub mod response;

#[derive(Default)]
pub struct OllamaImporter {
    prompt: Option<String>,
    connection: Option<OllamaConnection>,
}

impl Importer for OllamaImporter {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "info",
            Value::String("Not implemented yet".to_string()),
        );

        handler.handle_record(&mut record)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rig::{completion::Prompt, providers::ollama};

    use crate::importers::ollama::response::OllamaResponse;

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
}
