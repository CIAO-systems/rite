//! Module for file related functions to deal with the XML model

use super::Rite;
use std::{collections::HashMap, fs::File, io::Read};
use substitute::replace_env_variables;

mod substitute;

/// Parses the XML file and returns a [Rite] struct or a [std::error::Error]
/// It replaces all the variables found in the string with the values from the `variables` or system
/// environment variables. It calls [load_and_substitute_from_env]
///
/// # Arguments
/// * `xml_file` - The filename for the XML file to be loaded. Must be a \<rite\> XML
/// * `variables` -  Additional variables that can be substituted. 
/// 
/// # Example
/// ```
/// use std::collections::HashMap;
/// use model::xml::file::create_rite;
/// if let Ok(rite) = create_rite("example.xml", &HashMap::new()) {
///     println!("{:#?}", rite);
/// }
/// ```
pub fn create_rite(
    xml_file: &str,
    variables: &HashMap<String, String>,
) -> Result<Rite, Box<dyn std::error::Error>> {
    let xml_contents = load_and_substitute_from_env(xml_file, variables)?;

    let rite: Rite = match serde_xml_rs::from_str(&xml_contents) {
        Ok(rite) => rite,
        Err(e) => return Err(format!("Cannot parse contents from {}: {}", xml_file, e).into()),
    };
    Ok(rite)
}

/// Parses the XML file and returns a string of the contents of the file or a [std::error::Error]
/// It replaces all the variables found in the string with the values from the `variables` or system
/// environment variables
///
/// # Arguments
/// * `xml_file` - The filename for the XML file to be loaded. Must be a \<rite\> XML
/// * `variables` -  Additional variables that can be substituted. 
/// 
/// # Example
/// ```
/// use std::collections::HashMap;
/// use model::xml::file::load_and_substitute_from_env;
/// 
/// if let Ok(xml_contents) = load_and_substitute_from_env("example.xml", &HashMap::new()) {
///     println!("{}", xml_contents);
/// }
/// ```
pub fn load_and_substitute_from_env(
    xml_file: &str,
    variables: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
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
    let xml_contents = replace_env_variables(xml_contents, variables)?;
    Ok(xml_contents)
}
