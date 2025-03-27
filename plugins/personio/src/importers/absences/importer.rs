use import::Importer;

impl Importer for super::Absences {
    fn read(
        &mut self,
        _handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }
}
