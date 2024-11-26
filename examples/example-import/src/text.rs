use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

use import::Importer;
use model::{field::Field, record::Record};

#[derive(Debug)]
pub struct TextFileImporter {
    file_name: String,
    reader: Option<BufReader<File>>,
    next_line: usize,
}

impl TextFileImporter {
    pub fn new(file_name: String) -> Self {
        TextFileImporter {
            file_name,
            reader: None,
            next_line: 0,
        }
    }

    fn read_lines<F: FnMut(Record)>(
        &mut self,
        n: Option<usize>,
        mut callback: F,
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

                        callback(record);

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
}

impl Importer for TextFileImporter {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.file_name);
        let file = File::open(path)?;
        self.reader = Some(BufReader::new(file));
        self.next_line = 0;
        Ok(())
    }

    fn next(
        &mut self,
        n: Option<usize>,
    ) -> Result<Option<Vec<model::record::Record>>, Box<dyn std::error::Error>> {
        let mut records = Vec::new();
        self.read_lines(n, |record| records.push(record))?;
        Ok(Some(records))
    }

    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(reader) = self.reader.as_mut() {
            let _ = reader.seek(std::io::SeekFrom::Start(0))?;
            self.next_line = 0;
        }
        Ok(())
    }

    fn read(&mut self, callback: &mut dyn FnMut(Record)) -> Result<(), Box<dyn std::error::Error>> {
        self.read_lines(None, callback)?;
        Ok(())
    }
}

#[cfg(test)]
mod test;
