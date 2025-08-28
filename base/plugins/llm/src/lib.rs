use importers::ollama::OllamaImporter;
use importers::gemini::GeminiImporter;
use model::BoxedError;


pub mod common;
pub mod importers;

/// This functions creates an importer for the fake record generator
///
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn model::import::Importer>, BoxedError> {
    log::info!("Loading importer {name}");
    match name {
        "ollama" => Ok(Box::new(OllamaImporter::default())),
        "gemini" => Ok(Box::new(GeminiImporter::default())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use rig::{
        completion::Prompt,
        providers::{
            anthropic::{CLAUDE_3_SONNET, ClientBuilder},
            gemini,
        },
    };

    #[tokio::test]
    #[ignore = "for manual testing"]
    async fn test_claude() -> Result<(), Box<dyn std::error::Error>> {
        dotenv()?;
        let api_key = std::env::var("ANTHROPIC_API_KEY")?;

        let client = ClientBuilder::new(&api_key)
            .anthropic_version("2023-06-01")
            .anthropic_beta("prompt-caching-2024-07-31")
            .build();

        // create an agent directly
        let agent = client
            .agent(CLAUDE_3_SONNET)
            .preamble("You are a helpful assistant")
            .build();

        let answer = agent.prompt("What is the capitol of France?").await?;
        println!("Claudes answer:\n{answer}");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "for manual testing"]
    async fn test_gemini() -> Result<(), Box<dyn std::error::Error>> {
        dotenv()?;
        let api_key = std::env::var("GEMINI_API_KEY")?;

        // Initialize the Gemini client
        // Rig will automatically select a suitable Gemini model if not specified.
        // You can specify a model like .model("gemini-1.5-flash") or .model("gemini-1.5-pro")
        let gemini_client = gemini::Client::new(&api_key);

        // Create an agent using the Gemini client
        // An agent can be configured with a preamble (system prompt) for context.
        let agent = gemini_client
        .agent("gemini-1.5-flash") // You can specify a model here
        .preamble("You are a helpful and creative AI assistant. Provide concise and informative answers.")
        .build();

        // Prompt the Gemini model
        let response = agent.prompt("Tell me a fun fact about Germany.").await?;

        // Print the model's response
        println!("Gemini: {}", response);

        Ok(())
    }
}
