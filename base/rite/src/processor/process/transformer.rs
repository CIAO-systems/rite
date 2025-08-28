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
