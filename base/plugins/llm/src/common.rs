use model::{field::add_field, record::Record, xml::config::Configuration};

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

pub fn extract_json_structures(input: &str) -> Vec<serde_json::Value> {
    let mut json_structures = Vec::new();
    let mut stack = Vec::new();
    let mut start_index = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '{' | '[' => {
                if stack.is_empty() {
                    start_index = i;
                }
                stack.push(c);
            }
            '}' | ']' => {
                if let Some(_) = stack.pop() {
                    if stack.is_empty() {
                        let json_str = &input[start_index..=i + 1];
                        match serde_json::from_str::<serde_json::Value>(json_str) {
                            Ok(value) => {
                                json_structures.push(value);
                            }
                            Err(e) => {
                                log::error!("{e}");
                                eprintln!("{e}");
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    json_structures
}

pub fn handle_response(
    response: String,
    handler: &mut dyn import::RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let objects = extract_json_structures(&response);
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
