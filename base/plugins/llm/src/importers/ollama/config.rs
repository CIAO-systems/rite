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
    fn from(config: &Configuration) -> Result<OllamaConnection, BoxedError> {
        let client = match config.get(CFG_OLLAMA_URL) {
            Some(url) => ollama::Client::from_url(&url),
            None => ollama::Client::new(),
        };

        let llm_config = LLMConfiguration::from(config);

        let agent = match llm_config.agent() {
            Some(agent) => client
                .agent(&agent)
                .preamble(crate::common::system_prompt())
                .build(),
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

impl Initializable for OllamaImporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            let ollama_connection = OllamaConnection::from(&config)?;
            self.prompt = ollama_connection.config.prompt();
            self.connection = Some(ollama_connection);
        }

        Ok(())
    }
}
