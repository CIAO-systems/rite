use std::any::type_name;

use config::mapper::Mapper;
use model::{
    field::Field, record::Record, value::Value, xml::config::Configuration, Initializable,
};
use model::transform::Transformer;

mod config;

pub struct MapperTransformer {
    config: Option<Configuration>,
    mapper: Option<Mapper>,
}

impl<'a> MapperTransformer {
    pub fn new() -> Self {
        Self {
            config: None,
            mapper: None,
        }
    }
}

impl Initializable for MapperTransformer {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            if let Some(ref xml_file) = config.xml {
                self.mapper = Some(Mapper::new(&xml_file)?);
            }
            self.config = Some(config);
        }

        Ok(())
    }
}

impl Transformer for MapperTransformer {
    fn process(
        &self,
        record: &model::record::Record,
    ) -> Result<model::record::Record, model::BoxedError> {
        let result = if let Some(ref mapper) = self.mapper {
            let mut result = Record::new();

            for field in record.fields() {
                let mut new_field: Option<Field> = None;
                if let Some(mapping_field) = mapper.get(String::from(field.name())) {
                    // We found a mapping for this field
                    new_field = map_field(&mapping_field, field, Some(&record));
                }

                let f = if let Some(field_to_add) = new_field {
                    // Add the mapped field
                    field_to_add
                } else {
                    // Add the original field to the result
                    field.clone()
                };

                result.fields_as_mut().push(f);
            }
            result
        } else {
            Record::copy(record)
        };

        Ok(result)
    }
}

/// Maps the field value of `field` to the target name/value/type of `mapping_field`
/// # Arguments
/// * `mapping_field`: The configurtion for a mapping for a field with source   
///                    name matches with `field`
/// * `field`: The source field from the importer, which value should be mapped
///
fn map_field(
    mapping_field: &config::Field,
    field: &Field,
    record: Option<&Record>,
) -> Option<Field> {
    if let Some(ref values) = mapping_field.values {
        map_by_values(mapping_field, field, values)
    } else if let Some(ref patterns) = mapping_field.patterns {
        map_by_patterns(mapping_field, field, patterns, record)
    } else {
        None
    }
}

fn map_by_patterns(
    mapping_field_config: &config::Field,
    soure_field: &Field,
    patterns: &config::pattern::Patterns,
    record: Option<&Record>,
) -> Option<Field> {
    let source_value = soure_field.value().to_string();

    let replaced_value = patterns.apply(&source_value, record);

    let target_name = mapping_field_config.name.target.clone();
    Some(Field::new_value(
        &target_name,
        Value::String(replaced_value),
    ))
}

fn map_by_values(
    mapping_field_config: &config::Field,
    soure_field: &Field,
    values: &config::values::Values,
) -> Option<Field> {
    // Look for a matching mapping configuration for the `field`
    let source_value = soure_field.value();

    if let Some(mapping_value) = values.get(source_value.to_string()) {
        // We found a mapping field. Now we take the target value, convert it to the
        // the target type and create a new field with the target name
        let target_name = mapping_field_config.name.target.clone();
        let target_value: Value = convert_value(mapping_field_config, mapping_value);
        let mapped_field = Field::new_value(&target_name, target_value);

        Some(mapped_field)
    } else {
        None
    }
}

/// Converts the target value based on the target type
/// # Arguments
/// * `mapping_field`: The configuration for the mapping
/// * `mapping_value`: The value, that matched the source from the import
///
fn convert_value(mapping_field: &config::Field, mapping_value: config::values::Value) -> Value {
    match mapping_field.field_type.target.as_str() {
        "string" => {
            // convert mapping_value.target from String to String
            Value::String(mapping_value.target)
        }
        "i32" => parse::<i32, _>(&mapping_value.target, Value::I32),
        "i64" => parse::<i64, _>(&mapping_value.target, Value::I64),
        "f32" => parse::<f32, _>(&mapping_value.target, Value::F32),
        "f64" => parse::<f64, _>(&mapping_value.target, Value::F64),
        _ => Value::None,
    }
}

/// Takes a string value and returns a `Value` based on the `parse` result
/// # Arguments
/// * `value`: The string value to parse as type `T`
/// * `convert`: The function, that returns a `Value` for the parse result
///
/// # Example
/// ```
/// fn main() {
///     println!("Value from the string 47: {:?}", parse::<i32, _>("47", Value::I32));
/// }
/// ```
fn parse<T, F>(value: &str, convert: F) -> Value
where
    T: std::str::FromStr,
    F: FnOnce(T) -> Value,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match value.parse::<T>() {
        Ok(v) => convert(v),
        Err(e) => {
            log::error!("Error parsing {} as {}: {}", value, type_name::<T>(), e);
            Value::None
        }
    }
}

#[cfg(test)]
mod tests;
