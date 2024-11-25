use std::error::Error;

use model::record::Record;

pub trait Importer {
    /// Initializes the importer
    fn init(&mut self) -> Result<(), Box<dyn Error>>;

    /// Returns the next `n` records or None. If `n` is `None` the method must
    /// return all records
    fn next(
        &mut self,
        n: Option<usize>,
    ) -> Result<Option<Vec<model::record::Record>>, Box<dyn Error>>;

    /// Reads all from the import source and calls the `callback` for each record
    fn read<F: FnMut(Record)>(&mut self, callback: F) -> Result<(), Box<dyn std::error::Error>>;

    /// Resets the importer, so that `next` and `read` start from the beginning again
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub mod builtin;

#[cfg(test)]
mod test;
