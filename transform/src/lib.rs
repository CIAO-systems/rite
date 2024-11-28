use std::error::Error;

use model::{record::Record, Initializable};

pub trait Transformer: Initializable {
    fn process(&self, record: &Record) -> Result<Record, Box<dyn Error>>;
}
