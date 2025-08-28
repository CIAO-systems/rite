use model::export::Signal;
use model::import::RecordHandler;
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

    pub fn event(&mut self, signal: Signal) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut exporter) = self.exporter {
            exporter.event(signal)?;
        }

        Ok(())
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use model::export::{Exporter, Signal};
    use model::Initializable;

    use super::TransformAndExportRecordHandler;

    #[derive(Clone)]
    struct SignalTestExporter {
        pub signals: Rc<RefCell<Vec<Signal>>>,
    }

    impl SignalTestExporter {
        pub fn new() -> Self {
            Self {
                signals: Rc::new(RefCell::new(Vec::new())),
            }
        }
    }

    impl Initializable for SignalTestExporter {
        fn init(
            &mut self,
            _config: Option<model::xml::config::Configuration>,
        ) -> Result<(), model::BoxedError> {
            Ok(())
        }
    }

    impl Exporter for SignalTestExporter {
        fn write(&mut self, _record: &model::record::Record) -> Result<(), model::BoxedError> {
            Ok(())
        }

        fn event(
            &mut self,
            #[allow(unused_variables)] signal: model::export::Signal,
        ) -> Result<(), model::BoxedError> {
            self.signals.borrow_mut().push(signal);
            Ok(())
        }
    }

    #[test]
    fn test_signaling() {
        use crate::processor::process::exporter::Exporter;

        let mock = SignalTestExporter::new();

        let transformer = None;
        let mut v: Vec<Box<dyn model::export::Exporter>> = Vec::new();
        let b: Box<dyn model::export::Exporter> = Box::new(mock.clone());
        v.push(b);
        let mut exporter = Some(Exporter::new(&mut v));
        let mut subject = TransformAndExportRecordHandler::new(&transformer, &mut exporter);
        assert!(subject.event(Signal::Start).is_ok());
        assert!(subject.event(Signal::End).is_ok());
        assert_eq!(mock.signals.borrow().len(), 2);
        assert_eq!(mock.signals.borrow()[0], Signal::Start );
        assert_eq!(mock.signals.borrow()[1], Signal::End );
    }
}
