use std::collections::HashMap;

use subst::VariableMap;
use xml::reader::XmlEvent as ReaderEvent;
use xml::writer::XmlEvent as WriteEvent;
use xml::{EventReader, EventWriter};

pub struct VariablesAndEnv {
    variables: HashMap<String, String>,
    env: subst::Env,
}

impl VariablesAndEnv {
    // pub fn new() -> Self {
    //     Self {
    //         variables: HashMap::new(),
    //         env: subst::Env,
    //     }
    // }

    pub fn from(variables: &HashMap<String, String>) -> Self {
        Self {
            variables: variables.clone(),
            env: subst::Env,
        }
    }

    // pub fn insert(&mut self, key: &str, value: String) {
    //     self.variables.insert(String::from(key), value);
    // }
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

fn substitute_with_env(text: &str, variables: &VariablesAndEnv) -> String {
    // Replace environment variables
    match subst::substitute(&text, variables) {
        Ok(substituted) => substituted,
        Err(_) => String::from(text),
    }
}

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

    use crate::xml::file::substitute::replace_env_variables;

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
}
