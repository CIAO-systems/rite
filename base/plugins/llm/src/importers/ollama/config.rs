use model::{BoxedError, Initializable, xml::config::Configuration};
use rig::{
    agent::Agent,
    providers::ollama::{self, CompletionModel},
};
use tokio::runtime::Runtime;

use crate::common::LLMConfiguration;

use super::OllamaImporter;

const CFG_OLLAMA_URL: &str = "url";

pub struct OllamaConnection {
    pub runtime: Runtime,
    pub config: LLMConfiguration,
    pub agent: Agent<CompletionModel>,
}

impl OllamaConnection {
    fn from_config(config: &Configuration) -> Result<OllamaConnection, BoxedError> {
        let client = match config.get(CFG_OLLAMA_URL) {
            Some(url) => ollama::Client::from_url(&url),
            None => ollama::Client::new(),
        };

        let llm_config = LLMConfiguration::from(config);

        let agent = match llm_config.agent() {
            Some(agent) => client.agent(&agent).preamble(system_prompt()).build(),
            None => {
                return Err(format!("Configuration variable 'agent' must be set").into());
            }
        };

        let runtime = Runtime::new()?;

        Ok(OllamaConnection {
            runtime,
            agent,
            config: llm_config,
        })
    }
}

fn system_prompt() -> &'static str {
    r#"
    You are a JSON generator that always returns a raw JSON array with JSON 
    records of key/value pairs. Do not use markdown and give no comments.
    "#
}

impl Initializable for OllamaImporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            let ollama_connection = OllamaConnection::from_config(&config)?;
            self.prompt = ollama_connection.config.prompt();
            self.connection = Some(ollama_connection);
        }

        Ok(())
    }
}
