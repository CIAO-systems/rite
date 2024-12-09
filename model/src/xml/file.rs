use super::Rite;
use std::{fs::File, io::Read};
use substitute::replace_env_variables;

mod substitute;

/// Parses the XML file and returns a [Rite] struct or an [std::error::Error]
///
pub fn create_rite(xml_file: &str) -> Result<Rite, Box<dyn std::error::Error>> {
    let xml_contents = load_and_substitute_from_env(xml_file)?;

    let rite: Rite = match serde_xml_rs::from_str(&xml_contents) {
        Ok(rite) => rite,
        Err(e) => return Err(format!("Cannot parse contents from {}: {}", xml_file, e).into()),
    };
    Ok(rite)
}

pub fn load_and_substitute_from_env(xml_file: &str) -> Result<String, Box<dyn std::error::Error>> {
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
    let xml_contents = replace_env_variables(xml_contents)?;
    Ok(xml_contents)
}
