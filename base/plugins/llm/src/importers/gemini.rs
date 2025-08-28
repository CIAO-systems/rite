use crate::{
    common::{handle_response, response::LLMResponse},
    importers::gemini::config::GeminiConnection,
};
use model::import::Importer;
use model::BoxedError;
use rig::completion::Prompt;

mod config;

#[derive(Default)]
pub struct GeminiImporter {
    prompt: Option<String>,
    connection: Option<GeminiConnection>,
}

impl Importer for GeminiImporter {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

                let response = LLMResponse::new(&response)?;
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
