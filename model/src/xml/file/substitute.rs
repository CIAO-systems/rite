use xml::reader::XmlEvent as ReaderEvent;
use xml::writer::XmlEvent as WriteEvent;
use xml::{EventReader, EventWriter};

fn substitute_with_env(text: &str) -> String {
    // Replace environment variables
    match subst::substitute(&text, &subst::Env) {
        Ok(substituted) => substituted,
        Err(_) => String::from(text),
    }
}

pub(crate) fn replace_env_variables(
    xml_contents: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut output = Vec::new();
    // Create XML reader and writer
    let parser = EventReader::from_str(&xml_contents);
    let mut writer = EventWriter::new(&mut output);

    for event in parser {
        match event? {
            ReaderEvent::Characters(text) => {
                // Substitute variables in `text`
                let substituted = substitute_with_env(&text);
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
                            a.value = substitute_with_env(&a.value).to_string();
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
    use std::env::{remove_var, set_var};

    use crate::xml::file::substitute::replace_env_variables;

    #[test]
    fn test_subsitution() -> Result<(), Box<dyn std::error::Error>> {
        // Test 1
        remove_var("ELEMENT");
        remove_var("ATTRIBUTE");

        let input_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>${ELEMENT:replaced element}</element><attribute value="${ATTRIBUTE:replaced attribute}" /></example>"#;
        let expected_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>replaced element</element><attribute value="replaced attribute" /></example>"#;

        let result = replace_env_variables(input_xml.to_string())?;
        println!("{}", result);
        assert_eq!(expected_xml, result);

        // Test 2

        let input_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>${ELEMENT:replaced element}</element><attribute value="${ATTRIBUTE:replaced attribute}" /></example>"#;
        let expected_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>element from environment</element><attribute value="attribute from environment" /></example>"#;

        set_var("ELEMENT", "element from environment");
        set_var("ATTRIBUTE", "attribute from environment");
        let result = replace_env_variables(input_xml.to_string())?;
        println!("{}", result);
        assert_eq!(expected_xml, result);

        Ok(())
    }
}
