//! Module for text substitution

use std::collections::HashMap;

use subst::VariableMap;
use xml::reader::XmlEvent as ReaderEvent;
use xml::writer::XmlEvent as WriteEvent;
use xml::{EventReader, EventWriter};

/// A comination of user defined variables en system environment
pub struct VariablesAndEnv {
    variables: HashMap<String, String>,
    env: subst::Env,
}

impl VariablesAndEnv {
    /// Creates a [VariablesAndEnv] from a map of variables
    pub fn from(variables: &HashMap<String, String>) -> Self {
        Self {
            variables: variables.clone(),
            env: subst::Env,
        }
    }
}

impl VariableMap<'_> for VariablesAndEnv {
    type Value = String;

    fn get(&self, key: &str) -> Option<Self::Value> {
        if let Some(result) = self.variables.get(key) {
            return Some(result.clone());
        }

        self.env.get(key)
    }
}

/// Replaces variables in `text` with values from `variables`
///
/// # Arguments
/// * `text` - The text, that containes variables to be subsituted
/// * `variables` - A [VariableMap] that contains variables to substitue placeholders in in `text`
///
/// In case of any error, the `text` is returned as is
fn substitute_with_env(text: &str, variables: &VariablesAndEnv) -> String {
    // Replace environment variables
    match subst::substitute(&text, variables) {
        Ok(substituted) => substituted,
        Err(_) => String::from(text),
    }
}

/// Replaces all placeholders in `xml_contents` with values from `variables`
///
/// # Arguments
/// * `xml_contents` - The text with placeholders
/// * `variables` - A key/value [HashMap] which contains values to replace
///     placeholders in `xml_contents`
pub(crate) fn replace_env_variables(
    xml_contents: String,
    variables: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut output = Vec::new();
    // Create XML reader and writer
    let parser = EventReader::from_str(&xml_contents);
    let mut writer = EventWriter::new(&mut output);

    let variables = VariablesAndEnv::from(variables);

    for event in parser {
        match event? {
            ReaderEvent::Characters(text) => {
                // Substitute variables in `text`
                let substituted = substitute_with_env(&text, &variables);
                writer.write(WriteEvent::Characters(&substituted))?;
            }
            ReaderEvent::StartElement {
                ref name,
                ref mut attributes,
                ref namespace,
            } => {
                let element = WriteEvent::StartElement {
                    name: name.borrow(),
                    attributes: attributes
                        .iter_mut()
                        .map(|a| {
                            a.value = substitute_with_env(&a.value, &variables).to_string();
                            a.borrow()
                        })
                        .collect(),
                    namespace: namespace.borrow(),
                };
                writer.write(element)?;
            }
            other_event => {
                if let Some(writer_event) = other_event.as_writer_event() {
                    writer.write(writer_event)?;
                }
            }
        }
    }

    Ok(String::from_utf8(output)?)
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        env::{remove_var, set_var},
    };

    use crate::xml::file::substitute::{
        replace_env_variables, substitute_with_env, VariablesAndEnv,
    };

    #[test]
    fn test_subsitution() -> Result<(), Box<dyn std::error::Error>> {
        let mut variables: HashMap<String, String> = HashMap::new();
        variables.insert(String::from("KEY"), String::from("Value"));

        // Test 1
        remove_var("ELEMENT");
        remove_var("ATTRIBUTE");

        let input_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>$KEY: ${ELEMENT:replaced element}</element><attribute value="${ATTRIBUTE:replaced attribute}" /></example>"#;
        let expected_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>Value: replaced element</element><attribute value="replaced attribute" /></example>"#;

        let result = replace_env_variables(input_xml.to_string(), &variables)?;
        println!("{}", result);
        assert_eq!(expected_xml, result);

        // Test 2

        let input_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>${ELEMENT:replaced element}</element><attribute value="${ATTRIBUTE:replaced attribute}" /></example>"#;
        let expected_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>element from environment</element><attribute value="attribute from environment" /></example>"#;

        set_var("ELEMENT", "element from environment");
        set_var("ATTRIBUTE", "attribute from environment");
        let result = replace_env_variables(input_xml.to_string(), &variables)?;
        println!("{}", result);
        assert_eq!(expected_xml, result);

        Ok(())
    }

    #[test]
    fn test_substitute_with_error() {
        let text_with_error = "This text contains an incorrect variable: ${}. On parse error, the original text should be returned";
        let empty_variables: HashMap<String, String> = HashMap::new();

        // Call the function with input that will cause an error in the substitute function.
        let result = substitute_with_env(text_with_error, &VariablesAndEnv::from(&empty_variables));

        // Assert that the original text is returned when an error occurs.
        assert_eq!(result, text_with_error);
    }

    #[test]
    fn test_substitute_with_error_and_var() {
        let text_with_error = "This text contains an incorrect variable: ${} and a correct one: ${KEY}";
        let mut variables: HashMap<String, String> = HashMap::new();
        variables.insert(String::from("KEY"), String::from("Value"));

        // Call the function with input that will cause an error in the substitute function.
        let result = substitute_with_env(text_with_error, &VariablesAndEnv::from(&variables));

        // Assert that the original text is returned when an error occurs.
        assert_eq!(result, text_with_error);
    }

}
