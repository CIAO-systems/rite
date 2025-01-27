use import::RecordHandler;
use model::{record::Record, BoxedError};

use crate::processor::process::{exporter::Exporter, transformer::Transformer};

pub struct TransformAndExportRecordHandler<'a> {
    transformer: &'a Option<Transformer<'a>>,
    exporter: &'a mut Option<Exporter<'a>>,
}

impl<'a> TransformAndExportRecordHandler<'a> {
    pub(crate) fn new(
        transformer: &'a Option<Transformer<'a>>,
        exporter: &'a mut Option<Exporter<'a>>,
    ) -> Self {
        Self {
            transformer,
            exporter,
        }
    }
}

impl<'a> RecordHandler for TransformAndExportRecordHandler<'a> {
    fn handle_record(&mut self, record: &mut Record) -> Result<(), BoxedError> {
        import_read_handler(self.transformer, self.exporter, record);
        Ok(())
    }
}

fn import_read_handler<'a>(
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
