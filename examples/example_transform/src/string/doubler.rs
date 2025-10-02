//! Another example transformer for doubling characters of a string
use model::{field::Field, record::Record, Initializable};
use model::transform::Transformer;

/// Converst string field values to double each character
/// For example, the string "Hello" will be converted to "HHeelllloo"
///
pub struct CharacterDoubler;
impl CharacterDoubler {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Initializable for CharacterDoubler {
    /// This transformer does not have any configuration
    fn init(
        &mut self,
        _config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Transformer for CharacterDoubler {
    fn process(
        &self,
        record: &model::record::Record,
    ) -> Result<model::record::Record, Box<dyn std::error::Error>> {
        let mut result = Record::new();
        for field in record.fields() {
            match field.value() {
                model::value::Value::String(value) => {
                    let converted = value
                        .chars()
                        .flat_map(|c| std::iter::repeat(c).take(2))
                        .collect();
                    result.fields_as_mut().push(Field::new_value(
                        field.name(),
                        model::value::Value::String(converted),
                    ));
                }
                _ => {
                    // clone the field into the result record
                    result
                        .fields_as_mut()
                        .push(Field::new_value(field.name(), field.value()));
                }
            }
        }

        Ok(result)
    }
}
