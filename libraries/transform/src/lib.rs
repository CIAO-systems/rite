use model::{record::Record, BoxedError, Initializable};

pub trait Transformer: Initializable {
    /// Transforms the `record` and returns a new [Record]
    ///
    fn process(&self, record: &Record) -> Result<Record, BoxedError>;
}
