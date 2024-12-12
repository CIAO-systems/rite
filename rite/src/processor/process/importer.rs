use model::record::Record;

use super::{Exporter, Transformer};

pub struct Importer<'a> {
    importer: &'a mut Box<dyn import::Importer>,
}

impl<'a> Importer<'a> {
    pub fn new(importer: &'a mut Box<dyn import::Importer>) -> Self {
        Self { importer }
    }

    pub fn import(
        &'a mut self,
        transformer: &Option<Transformer<'a>>,
        exporter: &mut Option<Exporter<'a>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = self.importer.read(&mut |record| {
            import_read_callback(&transformer, exporter, record);
        }) {
            log::error!("Error while importing records: {}", e);
            Err(e)
        } else {
            Ok(())
        }
    }
}

fn import_read_callback<'a>(
    transformer: &Option<Transformer<'a>>,
    exporter: &mut Option<Exporter<'a>>,
    record: &Record,
) {
    let modified_record; // for it to life past the if/match blocks
    let transformed_record: &Record = if let Some(ref transformer) = transformer {
        match transformer.transform(record) {
            Ok(transformed_record) => {
                if let Some(transformed_record) = transformed_record {
                    modified_record = transformed_record;
                    &modified_record
                } else {
                    record
                }
            }
            Err(e) => {
                log::error!("Error while transforming records: {}", e);
                record
            }
        }
    } else {
        record
    };

    if let Some(ref mut exporter) = exporter {
        if let Err(e) = exporter.export(transformed_record) {
            log::error!("Error while exporting records: {}", e);
        }
    }
}
