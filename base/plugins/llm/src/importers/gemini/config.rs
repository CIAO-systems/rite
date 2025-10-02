use crate::{common::LLMConfiguration, importers::gemini::GeminiImporter};
use model::{BoxedError, Initializable, xml::config::Configuration};
use rig::{
    agent::Agent,
    providers::gemini::{self, completion::CompletionModel},
};
use tokio::runtime::Runtime;

pub struct GeminiConnection {
    pub runtime: Runtime,
    config: LLMConfiguration,
    pub agent: Agent<CompletionModel>,
}

impl Initializable for GeminiImporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            let connection = GeminiConnection::from(&config)?;
            self.prompt = connection.config.prompt();
            self.connection = Some(connection);
        }

        Ok(())
    }
}

const CFG_GEMINI_API_KEY: &str = "api_key";

impl GeminiConnection {
    fn from(config: &Configuration) -> Result<Self, BoxedError> {
        let api_key = config.get(CFG_GEMINI_API_KEY).ok_or(format!(
            "Configuration variable '{CFG_GEMINI_API_KEY}' must be set"
        ))?;

        let llm_config = LLMConfiguration::from(config);

        // Initialize the Gemini client
        let client = gemini::Client::new(&api_key);

        // Create an agent using the Gemini client
        // An agent can be configured with a preamble (system prompt) for context.
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
        Ok(GeminiConnection {
            runtime,
            config: llm_config,
            agent,
        })
    }
}
