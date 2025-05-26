use model::BoxedError;
use regex::Regex;

struct LLMResponse {
    pub thinking: Option<String>,
    pub response: Option<String>,
}

impl LLMResponse {
    pub fn new(input: &str) -> Result<Self, BoxedError> {
        let mut thinking: Option<String> = None;
        let mut response: Option<String> = None;
        let re = Regex::new(r"(?s)<think>(.*?)</think>(.*)")?;
        if let Some(captures) = re.captures(input) {
            thinking = captures.get(1).map(|m| m.as_str().to_string());
            response = captures.get(2).map(|m| m.as_str().to_string());
        }

        Ok(Self { thinking, response })
    }
}

#[cfg(test)]
mod tests {
    use rig::{completion::Prompt, providers::ollama};

    use crate::LLMResponse;

    #[tokio::test]
    async fn test_rig() -> Result<(), Box<dyn std::error::Error>> {
        // Create a new Ollama client (defaults to http://localhost:11434)
        let client = ollama::Client::new();

        // Create an agent
        let agent = client.agent("deepseek-r1:7b").build();

        let response = agent.prompt("Translate 'Hello, world!' to German.").await?;
        let response = LLMResponse::new(&response)?;
        if let Some(thinking) = response.thinking {
            println!("LLM was thinking about this:\n{thinking}\n\n")
        }
        if let Some(response) = response.response {
            println!("LLM response was:\n{response}");
        }

        Ok(())
    }
}
