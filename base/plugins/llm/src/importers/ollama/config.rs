use model::{BoxedError, Initializable, xml::config::Configuration};
use rig::{
    agent::Agent,
    providers::ollama::{self, CompletionModel},
};
use tokio::runtime::Runtime;

use super::OllamaImporter;

const CFG_OLLAMA_URL: &str = "url";
const CFG_OLLAMA_AGENT: &str = "agent";
const CFG_OLLAMA_PROMPT: &str = "prompt";

pub struct OllamaConnection {
    pub runtime: Runtime,
    pub agent: Agent<CompletionModel>,
}

impl OllamaConnection {
    fn from_config(config: &Configuration) -> Result<OllamaConnection, BoxedError> {
        let client = match config.get(CFG_OLLAMA_URL) {
            Some(url) => ollama::Client::from_url(&url),
            None => ollama::Client::new(),
        };

        let agent = match config.get(CFG_OLLAMA_AGENT) {
            Some(agent) => client.agent(&agent).preamble(system_prompt()).build(),
            None => {
                return Err(
                    format!("Configuration variable '{CFG_OLLAMA_AGENT}' must be set").into(),
                );
            }
        };

        let runtime = Runtime::new()?;

        Ok(OllamaConnection {
            runtime,
            agent,
        })
    }
}

fn system_prompt() -> &'static str {
    "You are a JSON generator that always returns a raw JSON array with JSON records of key/value pairs. Do not use markdown and give no comments."
}

impl Initializable for OllamaImporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.connection = Some(OllamaConnection::from_config(&config)?);
            self.prompt = config.get(CFG_OLLAMA_PROMPT);
        }

        Ok(())
    }
}
