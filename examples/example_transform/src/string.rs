use model::{field::Field, record::Record, Initializable};

use transform::Transformer;

pub enum StringFieldConversion {
    UpperCase,
    LowerCase,
}

pub struct StringFieldConverter {
    conversion: StringFieldConversion,
}

impl StringFieldConverter {
    pub fn new(conversion: StringFieldConversion) -> Self {
        StringFieldConverter { conversion }
    }
}

impl Initializable for StringFieldConverter {
    fn init(
        &mut self,
        _config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Transformer for StringFieldConverter {
    fn process(
        &self,
        record: &model::record::Record,
    ) -> Result<model::record::Record, Box<dyn std::error::Error>> {
        let mut result = Record::new();
        for field in record.fields() {
            match field.value() {
                model::value::Value::String(value) => {
                    let converted = match self.conversion {
                        StringFieldConversion::UpperCase => value.to_uppercase(),
                        StringFieldConversion::LowerCase => value.to_lowercase(),
                    };
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

pub mod doubler;

#[cfg(test)]
mod test;
