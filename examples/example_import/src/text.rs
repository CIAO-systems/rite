use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

use import::{Importer, RecordCallback};
use model::{field::Field, record::Record, xml, Initializable};

#[derive(Debug)]
pub struct TextFileImporter {
    config: Option<xml::config::Configuration>,
    reader: Option<BufReader<File>>,
    next_line: usize,
}

impl TextFileImporter {
    pub fn new() -> Self {
        TextFileImporter {
            config: None,
            reader: None,
            next_line: 0,
        }
    }

    fn read_lines(
        &mut self,
        n: Option<usize>,
        callback: RecordCallback,
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
                            .push(Field::new_string("line".to_string(), line));
                        record.fields_as_mut().push(Field::new_usize(
                            "index".to_string(),
                            self.next_line + index,
                        ));

                        callback(&record);

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

    pub fn next(
        &mut self,
        n: Option<usize>,
    ) -> Result<Option<Vec<model::record::Record>>, Box<dyn std::error::Error>> {
        let mut records: Vec<Record> = Vec::new();
        self.read_lines(n, &mut |record| {
            records.push(Record::copy(record));
        })?;
        Ok(Some(records))
    }
}

impl Importer for TextFileImporter {
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(reader) = self.reader.as_mut() {
            let _ = reader.seek(std::io::SeekFrom::Start(0))?;
            self.next_line = 0;
        }
        Ok(())
    }

    fn read(
        &mut self,
        callback: &mut dyn FnMut(&Record),
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.read_lines(None, callback)?;
        Ok(())
    }
}

impl Initializable for TextFileImporter {
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
