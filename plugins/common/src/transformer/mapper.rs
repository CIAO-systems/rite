use config::mapper::Mapper;
use model::{
    field::Field, record::Record, value::Value, xml::config::Configuration, Initializable,
};
use transform::Transformer;

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

            // FIXME use field_type (currently it is only string)
            for field in record.fields() {
                let mut new_field: Option<Field> = None;
                let name = String::from(field.name());
                if let Some(mapping_field) = mapper.get(name) {
                    // Add a field with the target name and the target value and type
                    if let Some(mapping_value) = mapping_field.values.get(field.value().to_string())
                    {
                        // Create a new field with the target name and target value
                        let mapped_field = Field::new_value(
                            mapping_field.name.target,
                            Value::String(mapping_value.target.to_string()),
                        );

                        new_field = Some(mapped_field);
                    }
                }

                if let Some(field_to_add) = new_field {
                    // Add the mapped field to the result
                    result.fields_as_mut().push(field_to_add);
                } else {
                    // Add the original field to the result
                    result.fields_as_mut().push(field.clone());
                }
            }
            result
        } else {
            Record::copy(record)
        };

        Ok(result)
    }
}
