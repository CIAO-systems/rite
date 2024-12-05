use std::{fs::File, io::Read};

use super::Rite;

pub fn create_rite(xml_file: &str) -> Result<Rite, Box<dyn std::error::Error>> {
    let mut file = match File::open(xml_file) {
        Ok(file) => file,
        Err(e) => return Err(format!("Cannot open {}: {}", xml_file, e).into()),
    };

    let mut xml_contents = String::new();
    match file.read_to_string(&mut xml_contents) {
        Ok(_) => { //ignore
        }
        Err(e) => return Err(format!("Cannot read contents from {}: {}", xml_file, e).into()),
    }

    // TODO: include variable substitution here

    let rite: Rite = match serde_xml_rs::from_str(&xml_contents) {
        Ok(rite) => rite,
        Err(e) => return Err(format!("Cannot parse contents from {}: {}", xml_file, e).into()),
    };
    Ok(rite)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use xml::reader::XmlEvent as ReaderEvent;
    use xml::writer::XmlEvent as WriteEvent;
    use xml::{EventReader, EventWriter};

    fn substitute(text: &str) -> String {
        // Example replacements
        let mut replacements = HashMap::new();
        replacements.insert("ELEMENT".to_string(), "replaced element".to_string());
        replacements.insert("ATTRIBUTE".to_string(), "replaced attribute".to_string());

        match subst::substitute(&text, &replacements) {
            Ok(substituted) => substituted,
            Err(_) => String::from(text),
        }
    }

    #[test]
    fn test_subsitution() -> Result<(), Box<dyn std::error::Error>> {
        let input_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>${ELEMENT}</element><attribute value="${ATTRIBUTE}" /></example>"#;
        let expected_xml = r#"<?xml version="1.0" encoding="UTF-8"?><example><element>replaced element</element><attribute value="replaced attribute" /></example>"#;

        let mut output = Vec::new();
        // Create XML reader and writer
        let parser = EventReader::from_str(input_xml);
        let mut writer = EventWriter::new(&mut output);

        for event in parser {
            match event? {
                ReaderEvent::Characters(text) => {
                    // Substitute variables in `text`
                    let substituted = substitute(&text);
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
                                a.value = substitute(&a.value).to_string();
                                a.borrow()
                            })
                            .collect(),
                        namespace: namespace.borrow(),
                    };
                    writer.write(element)?;
                }
                // ReaderEvent::StartElement {
                //     name,
                //     attributes,
                //     namespace,
                // } => {
                //     // FIXME: see, if other_event.as_writer_event() gives some clue,
                //     // how to implement this
                //     let substituted_attributes: Vec<xml::attribute::Attribute> = attributes
                //         .into_iter()
                //         .map(|attr| {
                //             WriteEvent::start_element(name.into())
                //                 .ns(attr.name.namespace.into(), "")
                //                 .attr(attr.name.into(), &substitute(&attr.value))
                //                 .into()
                //         })
                //         .collect();

                //     writer.write(WriteEvent::StartElement {
                //         name: name.borrow(),
                //         attributes: Cow::Owned(substituted_attributes),
                //         namespace: namespace.borrow(),
                //     })?;
                // }
                other_event => {
                    if let Some(writer_event) = other_event.as_writer_event() {
                        writer.write(writer_event)?;
                    }
                }
            }
        }

        let result = String::from_utf8(output)?;
        println!("{}", result);
        assert_eq!(expected_xml, result);
        Ok(())
    }
}
