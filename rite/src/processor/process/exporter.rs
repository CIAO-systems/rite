use model::record::Record;

pub struct Exporter<'a> {
    exporters: &'a mut Vec<Box<dyn export::Exporter>>,
}

impl<'a> Exporter<'a> {
    pub fn new(exporters: &'a mut Vec<Box<dyn export::Exporter>>) -> Self {
        Self { exporters }
    }

    /// Exports the record to all exporters
    ///
    pub fn export(&mut self, record: &Record) -> Result<(), Box<dyn std::error::Error>> {
        // Export to every configured exporter
        for exporter in self.exporters.iter_mut() {
            exporter.write(record)?;
        }

        Ok(())
    }
}
