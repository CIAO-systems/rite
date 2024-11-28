use model::{record::Record, Initializable};

pub trait Importer: Initializable {
    /// Returns the next `n` records or None. If `n` is `None` the method must
    /// return all records
    fn next(
        &mut self,
        n: Option<usize>,
    ) -> Result<Option<Vec<model::record::Record>>, Box<dyn std::error::Error>>;

    /// Reads all from the import source and calls the `callback` for each record
    fn read(&mut self, callback: &mut dyn FnMut(Record)) -> Result<(), Box<dyn std::error::Error>>;

    /// Resets the importer, so that `next` and `read` start from the beginning again
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
