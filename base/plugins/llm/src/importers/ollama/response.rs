use model::BoxedError;
use regex::Regex;

pub struct OllamaResponse {
    pub thinking: Option<String>,
    pub response: Option<String>,
}

impl OllamaResponse {
    pub fn new(input: &str) -> Result<Self, BoxedError> {
        let mut thinking: Option<String> = None;
        let response: Option<String>;
        let re = Regex::new(r"(?s)<think>(.*?)</think>(.*)")?;
        if let Some(captures) = re.captures(input) {
            thinking = captures.get(1).map(|m| m.as_str().to_string());
            response = captures.get(2).map(|m| m.as_str().to_string());
        } else {
            // If no <think> block is found, the entire input is the response
            response = Some(input.to_string());
        }

        Ok(Self { thinking, response })
    }
}

#[cfg(test)]
mod tests {
    use crate::importers::ollama::response::OllamaResponse;

    // Test case 1: Input with both <think> block and response
    #[test]
    fn test_with_think_and_response() {
        let input = "<think>I am thinking about the next step.</think>This is the actual response.";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(
            result.thinking,
            Some("I am thinking about the next step.".to_string())
        );
        assert_eq!(
            result.response,
            Some("This is the actual response.".to_string())
        );
    }

    // Test case 2: Input with only response (no <think> block)
    #[test]
    fn test_only_response() {
        let input = "This is just a direct response with no thinking.";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(result.thinking, None);
        assert_eq!(
            result.response,
            Some("This is just a direct response with no thinking.".to_string())
        );
    }

    // Test case 3: Input with an empty <think> block and a response
    #[test]
    fn test_empty_think_block_with_response() {
        let input = "<think></think>Here is the response after an empty thought.";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(result.thinking, Some("".to_string()));
        assert_eq!(
            result.response,
            Some("Here is the response after an empty thought.".to_string())
        );
    }

    // Test case 4: Input with <think> block and no text after it
    #[test]
    fn test_think_block_no_trailing_response() {
        let input = "<think>Just thinking, nothing more.</think>";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(
            result.thinking,
            Some("Just thinking, nothing more.".to_string())
        );
        assert_eq!(result.response, Some("".to_string())); // Response should be an empty string
    }

    // Test case 5: Empty input string
    #[test]
    fn test_empty_input() {
        let input = "";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(result.thinking, None);
        assert_eq!(result.response, Some("".to_string()));
    }

    // Test case 6: Input with multiple lines in thinking and response
    #[test]
    fn test_multiline_content() {
        let input = "<think>
This is line 1 of thought.
This is line 2 of thought.
</think>
This is line 1 of response.
This is line 2 of response.";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(
            result.thinking,
            Some("\nThis is line 1 of thought.\nThis is line 2 of thought.\n".to_string())
        );
        assert_eq!(
            result.response,
            Some("\nThis is line 1 of response.\nThis is line 2 of response.".to_string())
        );
    }

    // Test case 7: Input with leading whitespace before <think>
    #[test]
    fn test_leading_whitespace_before_think() {
        let input = "  <think>Thinking.</think>Response.";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(result.thinking, Some("Thinking.".to_string()));
        assert_eq!(result.response, Some("Response.".to_string()));
    }

    // Test case 8: Input with some text before <think> (should be treated as full response)
    #[test]
    fn test_text_before_think_block() {
        let input = "Some intro text.<think>Actual thought.</think>Response.";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(result.thinking, Some("Actual thought.".to_string()));
        assert_eq!(result.response, Some("Response.".to_string()));
    }

    // Test case 9: Input with malformed think tag (should be treated as full response)
    #[test]
    fn test_malformed_think_tag() {
        let input = "<think>Thinking.</thinkmissing>Response.";
        let result = OllamaResponse::new(input).unwrap();

        assert_eq!(result.thinking, None);
        assert_eq!(
            result.response,
            Some("<think>Thinking.</thinkmissing>Response.".to_string())
        );
    }
}
