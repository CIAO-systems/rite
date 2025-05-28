use model::{BoxedError, Initializable, xml::config::Configuration};
use rig::{
    agent::Agent,
    providers::ollama::{self, Client, CompletionModel},
};
use tokio::runtime::Runtime;

use super::OllamaImporter;

const CFG_OLLAMA_URL: &str = "url";
const CFG_OLLAMA_AGENT: &str = "agent";
const CFG_OLLAMA_PROMPT: &str = "prompt";

pub struct OllamaConnection {
    runtime: Runtime,
    client: Client,
    agent: Agent<CompletionModel>,
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
            client,
            agent,
        })
    }
}

fn system_prompt() -> &'static str {
    "Always answer in form of a raw (no markdown) JSON list of records with key/value pairs. Do not add any notes!"
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
