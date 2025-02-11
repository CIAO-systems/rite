//! Module for the example text file importer

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

use import::{handlers::CollectingRecordHandler, Importer, RecordHandler};
use model::{
    field::Field, record::Record, value::Value, xml::config::Configuration, BoxedError,
    Initializable,
};

/// An [Importer] that reads lines from a text file
///
/// The text file to read must be configured in a configuration key named `file_name`
#[derive(Debug)]
pub struct TextFileImporter {
    config: Option<Configuration>,
    reader: Option<BufReader<File>>,
    next_line: usize,
}

impl TextFileImporter {
    /// Creates an unconfigured [TextFileImporter]
    pub fn new() -> Self {
        TextFileImporter {
            config: None,
            reader: None,
            next_line: 0,
        }
    }

    /// Reads the (optional) next n lines from the text file and creates [Record]s.
    /// For every [Record], the callback is called
    /// # Arguments
    /// * `n` - an (optional) amount of how many lines should be read.
    ///     If it is [None], all remaining lines will be read
    /// * `callback` - The [Record] that is created for each line will be given
    ///     to the `callback`
    fn read_lines(
        &mut self,
        n: Option<usize>,
        handler: &mut dyn RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut index: usize = 0;

        if let Some(reader) = self.reader.as_mut() {
            // reader.lines() continues at the last position, if this function
            // was called previously already

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        index += 1;

                        let mut record = Record::new();
                        record
                            .fields_as_mut()
                            .push(Field::new_value("line", Value::String(line)));
                        record.fields_as_mut().push(Field::new_value(
                            "index",
                            Value::USize(self.next_line + index),
                        ));

                        handler.handle_record(&mut record)?;

                        if let Some(n) = n {
                            if index == n {
                                break;
                            }
                        }
                    }
                    Err(e) => return Err(Box::new(e)),
                }
            }
        } else {
            return Err("Not initialized".into());
        }

        self.next_line += index;
        Ok(())
    }

    /// Reads the nex `n` lines, creates a [Record] and returns a vector with all records
    ///
    /// # Arguments
    /// * `n` - If `n` is [None], all lines a read into [Record]s, otherwise the next n records will be read
    pub fn next(
        &mut self,
        n: Option<usize>,
    ) -> Result<Option<Vec<model::record::Record>>, Box<dyn std::error::Error>> {
        let mut records: Vec<Record> = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        self.read_lines(n, &mut handler)?;
        Ok(Some(records))
    }
}

impl Importer for TextFileImporter {
    /// Resets the importer to start importing from the beginning
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(reader) = self.reader.as_mut() {
            let _ = reader.seek(std::io::SeekFrom::Start(0))?;
            self.next_line = 0;
        }
        Ok(())
    }

    /// Reads the lines of the configured text file and calls the `callback` for each created [Record]
    /// # Arguments
    /// * `callback` - Function that will be called with the constructed [Record]
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
        self.read_lines(None, handler)?;
        Ok(())
    }
}

impl Initializable for TextFileImporter {
    /// Initializes the importer from the [Configuration]
    /// # Configuration
    /// * `file_name` - The name of the text file to read from
    fn init(&mut self, config: Option<Configuration>) -> Result<(), Box<dyn std::error::Error>> {
        // take ownership for `config`
        self.config = config;

        match self.config.as_ref() {
            Some(config) => match config.get("file_name") {
                Some(file_name) => {
                    let path = Path::new(&file_name);
                    let file = match File::open(path) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("Error opening file {}: {}", path.display(), e);
                            return Err(e.into());
                        }
                    };
                    self.reader = Some(BufReader::new(file));
                    self.next_line = 0;
                    Ok(())
                }
                None => Err("Missing configuration for 'file_name'".into()),
            },
            None => Err("Missing configuration for 'file_name'".into()),
        }
    }
}

#[cfg(test)]
mod tests;
