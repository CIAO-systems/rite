use model::{field::Field, record::Record, Initializable};
use transform::Transformer;

pub struct CharacterDoubler;
impl CharacterDoubler {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Initializable for CharacterDoubler {
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
                    result
                        .fields_as_mut()
                        .push(Field::new_string(field.name().to_string(), converted));
                }
                _ => {
                    // clone the field into the result record
                    result
                        .fields_as_mut()
                        .push(Field::new_value(field.name().to_string(), field.value()));
                }
            }
        }

        Ok(result)
    }
}
