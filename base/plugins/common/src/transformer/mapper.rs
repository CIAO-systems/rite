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
mod tests {
    use super::*;

    // Mock implementations for testing
    fn create_field(target_type: &str) -> config::Field {
        config::Field {
            name: config::Name {
                source: "source".to_string(),
                target: "target".to_string(),
            },
            field_type: config::Type {
                source: "source_type".to_string(),
                target: target_type.to_string(),
            },
            patterns: None,
            values: Some(config::values::Values { value: vec![] }),
        }
    }

    fn create_value(target: String) -> config::values::Value {
        config::values::Value {
            source: "source_value".to_string(),
            target,
        }
    }

    #[test]
    fn test_parse_successful_conversions() {
        // Test successful integer conversions
        assert!(matches!(parse::<i32, _>("42", Value::I32), Value::I32(42)));
        assert!(matches!(
            parse::<i64, _>("9223372036854775807", Value::I64),
            Value::I64(9223372036854775807)
        ));

        // Test successful float conversions
        assert!(
            matches!(parse::<f32, _>("3.14", Value::F32), Value::F32(x) if (x - 3.14).abs() < f32::EPSILON)
        );
        assert!(
            matches!(parse::<f64, _>("3.14159", Value::F64), Value::F64(x) if (x - 3.14159).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn test_parse_failed_conversions() {
        // Test parsing failures
        assert!(matches!(
            parse::<i32, _>("not a number", Value::I32),
            Value::None
        ));
        assert!(matches!(parse::<i64, _>("xyz", Value::I64), Value::None));
        assert!(matches!(parse::<f32, _>("hello", Value::F32), Value::None));
        assert!(matches!(parse::<f64, _>("world", Value::F64), Value::None));
    }

    #[test]
    fn test_convert_value_successful_conversions() {
        // Test string conversion
        let string_field = create_field("string");
        let string_value = create_value("hello".to_string());
        assert!(matches!(
            convert_value(&string_field, string_value),
            Value::String(s) if s == "hello"
        ));

        // Test integer conversions
        let i32_field = create_field("i32");
        let i32_value = create_value("42".to_string());
        assert!(matches!(
            convert_value(&i32_field, i32_value),
            Value::I32(42)
        ));

        let i64_field = create_field("i64");
        let i64_value = create_value("9223372036854775807".to_string());
        assert!(matches!(
            convert_value(&i64_field, i64_value),
            Value::I64(9223372036854775807)
        ));

        // Test float conversions
        let f32_field = create_field("f32");
        let f32_value = create_value("3.14".to_string());
        assert!(matches!(
            convert_value(&f32_field, f32_value),
            Value::F32(x) if (x - 3.14).abs() < f32::EPSILON
        ));

        let f64_field = create_field("f64");
        let f64_value = create_value("3.14159".to_string());
        assert!(matches!(
            convert_value(&f64_field, f64_value),
            Value::F64(x) if (x - 3.14159).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn test_convert_value_failed_conversions() {
        // Test failed conversions
        let i32_field = create_field("i32");
        let bad_i32_value = create_value("not a number".to_string());
        assert!(matches!(
            convert_value(&i32_field, bad_i32_value),
            Value::None
        ));

        let f64_field = create_field("f64");
        let bad_f64_value = create_value("invalid".to_string());
        assert!(matches!(
            convert_value(&f64_field, bad_f64_value),
            Value::None
        ));

        // Test unknown type
        let unknown_field = create_field("unknown_type");
        let unknown_value = create_value("some value".to_string());
        assert!(matches!(
            convert_value(&unknown_field, unknown_value),
            Value::None
        ));
    }
}
