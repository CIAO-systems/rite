use model::BoxedError;
use regex::Regex;

pub struct OllamaResponse {
    pub thinking: Option<String>,
    pub response: Option<String>,
}

impl OllamaResponse {
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
