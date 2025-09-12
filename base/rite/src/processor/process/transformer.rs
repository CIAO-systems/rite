use model::record::Record;

pub struct Transformer<'a> {
    transformers: &'a Vec<Box<dyn model::transform::Transformer>>,
}

impl<'a> Transformer<'a> {
    pub fn new(transformers: &'a Vec<Box<dyn model::transform::Transformer>>) -> Self {
        Self { transformers }
    }

    pub fn transform(&self, record: &Record) -> Result<Option<Record>, Box<dyn std::error::Error>> {
        let mut transformed_record = Record::copy(record);
        for transformer in self.transformers {
            transformed_record = transformer.process(&transformed_record)?;
        }
        Ok(Some(transformed_record))
    }
}

#[cfg(test)]
mod tests {
    use model::{
        field::add_field, transform, value::Value, Initializable
    };

    use super::*;

    struct TestTransformer;

    impl Initializable for TestTransformer {
        fn init(
            &mut self,
            _config: Option<model::xml::config::Configuration>,
        ) -> Result<(), model::BoxedError> {
            Ok(())
        }
    }
    
    impl transform::Transformer for TestTransformer {
        fn process(&self, record: &Record) -> Result<Record, model::BoxedError> {
            let mut result = record.clone();
            add_field(result.fields_as_mut(), "new_field", 73.into());
            Ok(result)
        }
    }

    #[test]
    fn test_new() {
        let transformers: Vec<Box<dyn transform::Transformer>> = Vec::new();
        let t = Transformer::new(&transformers);
        assert_eq!(t.transformers.len(), 0);
    }

    #[test]
    fn test_transform() {
        let mut transformers: Vec<Box<dyn transform::Transformer>> = Vec::new();
        transformers.push(Box::new(TestTransformer));

        let t = Transformer::new(&transformers);
        assert_eq!(t.transformers.len(), 1);

        let record = Record::new();
        let result = t.transform(&record);
        assert!(result.is_ok());

        let record = result.unwrap().unwrap();
        assert_eq!(record.fields().len(), 1);

        let field = record.field_by_name("new_field");
        assert!(field.is_some());
        let field = field.unwrap();
        assert_eq!(field.name(), "new_field");
        assert_eq!(field.value(), Value::I32(73));
    }
}
