use model::{record::Record, Initializable};

pub trait Importer: Initializable {
    /// Reads all from the import source and calls the `callback` for each record
    fn read(&mut self, callback: &mut dyn FnMut(Record)) -> Result<(), Box<dyn std::error::Error>>;

    /// Resets the importer, so that `next` and `read` start from the beginning again
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
