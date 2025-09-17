use model::xml::config::Configuration;

use crate::common::{
    CFG_AGENT, CFG_PROMPT, CFG_PROMPT_FILE, LLMConfiguration, extract_json_structures,
};

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

#[test]
fn test_from_empty_configuration() {
    let config = Configuration::new();
    let llm_config = LLMConfiguration::from(&config);

    assert!(llm_config.agent.is_none());
    assert!(llm_config.prompt.is_none());
}

#[test]
fn test_from_configuration() {
    let mut config = Configuration::new();
    config.insert_str(CFG_AGENT, "agent");
    config.insert_str(CFG_PROMPT, "prompt");
    let llm_config = LLMConfiguration::from(&config);

    assert!(llm_config.agent().is_some_and(|f| f == "agent"));
    assert!(llm_config.prompt().is_some_and(|f| f == "prompt"));
}

#[test]
fn test_from_configuration_promptfile() {
    let mut config = Configuration::new();
    const PROMPT_FILE: &str = "../../data/test/llm/prompt-gemini.txt";
    config.insert_str(CFG_PROMPT_FILE, PROMPT_FILE);
    let llm_config = LLMConfiguration::from(&config);

    let prompt = std::fs::read_to_string(PROMPT_FILE).unwrap();
    assert!(llm_config.prompt().is_some_and(|f| f == prompt));
}
