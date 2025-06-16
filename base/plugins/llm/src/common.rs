use model::{BoxedError, field::add_field, record::Record, xml::config::Configuration};

const CFG_AGENT: &str = "agent";
const CFG_PROMPT_FILE: &str = "prompt-file";
const CFG_PROMPT: &str = "prompt";

pub struct LLMConfiguration {
    agent: Option<String>,
    prompt: Option<String>,
}

impl From<&Configuration> for LLMConfiguration {
    fn from(config: &Configuration) -> Self {
        let agent = config.get(CFG_AGENT);
        let mut prompt = config.get(CFG_PROMPT);
        if let Some(prompt_file) = config.get(CFG_PROMPT_FILE) {
            if let Ok(content) = std::fs::read_to_string(prompt_file) {
                prompt = Some(content);
            }
        }

        Self { agent, prompt }
    }
}

impl LLMConfiguration {
    pub fn agent(&self) -> Option<String> {
        self.agent.clone()
    }

    pub fn prompt(&self) -> Option<String> {
        self.prompt.clone()
    }
}

pub fn extract_json_structures(input: &str) -> Result<Vec<serde_json::Value>, BoxedError> {
    let mut json_structures = Vec::new();
    let mut stack = Vec::new();
    let mut start_byte_index = 0;
    let mut in_json = false;

    for (byte_index, c) in input.char_indices() {
        match c {
            '{' | '[' => {
                if stack.is_empty() {
                    start_byte_index = byte_index;
                    in_json = true;
                }
                stack.push(c);
            }
            '}' | ']' => {
                if let Some(_) = stack.pop() {
                    if stack.is_empty() && in_json {
                        let json_str = &input[start_byte_index..=byte_index];
                        match serde_json::from_str::<serde_json::Value>(json_str) {
                            Ok(value) => {
                                json_structures.push(value);
                                in_json = false;
                            }
                            Err(e) => {
                                log::error!("{e}");
                                return Err(format!("Serde error: {e}").into());
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    Ok(json_structures)
}

pub fn handle_response(
    response: String,
    handler: &mut dyn import::RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let objects = extract_json_structures(&response)?;
    log::debug!("Extracted {} JSON structures", objects.len());

    for object in objects {
        match object {
            serde_json::Value::Array(values) => {
                for value in values {
                    if let Some(map) = value.as_object() {
                        let mut record = extract_record(map);
                        handler.handle_record(&mut record)?;
                    }
                }
            }
            serde_json::Value::Object(map) => {
                let mut record = extract_record(&map);
                handler.handle_record(&mut record)?;
            }
            _ => (),
        }
    }

    Ok(())
}

fn extract_record(map: &serde_json::Map<String, serde_json::Value>) -> Record {
    let mut record = Record::new();

    for (key, value) in map.iter() {
        log::debug!("{}={}", key, value);
        add_field(
            record.fields_as_mut(),
            key,
            model::value::Value::from(value.clone()),
        );
    }

    record
}

#[cfg(test)]
mod tests {
    use crate::common::extract_json_structures;

    const JSON: &str = r#"
[
    {"alter": 37, "first": "Manuel", "last": "Schmidt"},
    {"alter": 45, "first": "Sabine", "last": "Müller"},
    {"alter": 28, "first": "Patrick", "last": "Krause"},
    {"alter": 53, "first": "Erika", "last": "Schulz"},
    {"alter": 41, "first": "Jan", "last": "Brandt"},
    {"alter": 31, "first": "Carina", "last": "Wolff"},
    {"alter": 62, "first": "Oliver", "last": "Nicolai"},
    {"alter": 24, "first": "Lena", "last": "Köhler"},
    {"alter": 39, "first": "Andreas", "last": "Schmitt"},
    {"alter": 57, "first": "Monika", "last": "Meier"}
]
"#;

    #[test]
    fn test_extract_json_structures() {
        let objects = extract_json_structures(JSON);
        assert!(objects.is_ok());
        assert_eq!(1, objects.unwrap().len());
    }
}
