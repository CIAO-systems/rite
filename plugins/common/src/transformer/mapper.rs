use model::{record::Record, xml::config::Configuration, Initializable};
use transform::Transformer;

mod config;

pub struct MapperTransformer {
    config: Option<Configuration>,
}

impl MapperTransformer {
    pub(crate) fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for MapperTransformer {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        // FIXME implement me

        Ok(())
    }
}

impl Transformer for MapperTransformer {
    fn process(
        &self,
        record: &model::record::Record,
    ) -> Result<model::record::Record, model::BoxedError> {
        // FIXME implement me
        let result = Record::copy(record);
        
        Ok(result)
    }
}
