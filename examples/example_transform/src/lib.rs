//! Example plugin for a transformer

use string::StringFieldConverter;
use transform::Transformer;

pub mod string;

/// Plugin entry function to create an instance of an [Transformer]
/// # Arguments
/// * `name` - When the plugin supports multiple transformers, the `name` is used 
///     to determined, what transformer to return
/// # Available transformers
/// * `uppercase` - Converts every string field value to uppercase
/// * `lowercase` - Converts every string field value to lowercase
/// * `doubler` - Converts every string field value by doubling every character
#[no_mangle]
pub fn create_transformer(name: &str) -> Result<Box<dyn Transformer>, Box<dyn std::error::Error>> {
    match name {
        "uppercase" => Ok(Box::new(StringFieldConverter::new(
            string::StringFieldConversion::UpperCase,
        ))),
        "lowercase" => Ok(Box::new(StringFieldConverter::new(
            string::StringFieldConversion::LowerCase,
        ))),
        "doubler" => Ok(Box::new(string::doubler::CharacterDoubler::new())),
        _ => Err(format!("Unknown transformer '{name}'").into()),
    }
}
