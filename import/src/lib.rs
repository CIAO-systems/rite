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
}

pub mod builtin {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    use model::{field::Field, record::Record};

    use crate::Importer;

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
            let mut skipped = 0;
            let mut index: usize = 0;

            if let Some(reader) = self.reader.as_mut() {
                for line in reader.lines() {
                    if skipped < self.next_line {
                        skipped += 1;
                        continue;
                    }

                    match line {
                        Ok(line) => {
                            index += 1;

                            let mut record = Record::new();
                            record
                                .fields()
                                .push(Field::new_string("line".to_string(), line));
                            record.fields().push(Field::new_usize(
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

        fn read<F: FnMut(Record)>(
            &mut self,
            callback: F,
        ) -> Result<(), Box<dyn std::error::Error>> {
            self.read_lines(None, callback)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use model::{record::Record, value::Value};

    use crate::{builtin::TextFileImporter, Importer};

    fn check_correct_values(record: model::record::Record) {
        match record.field_by_name("index") {
            Some(field) => match field.value() {
                Value::USize(index) if index != 4 => {
                    check_line_value(&record, &format!("Line{}", index))
                }
                Value::USize(_index) => check_line_value(&record, ""),

                _ => panic!("Wrong datatype for index"),
            },
            _ => panic!("Field not found: index"),
        }
    }

    fn check_line_value(record: &model::record::Record, expected: &str) {
        match record.field_by_name("line") {
            Some(field_line) => match field_line.value() {
                Value::String(line) => {
                    assert_eq!(line, expected)
                }
                _ => panic!("Wrong datatype for line"),
            },
            _ => panic!("Field not found: line"),
        }
    }

    fn print_record(record: &Record) {
        println!(
            "{:?} = {:?}",
            record.field_by_name("index").unwrap().value(),
            record.field_by_name("line").unwrap().value()
        );
    }

    #[test]
    fn test_next() {
        let mut importer = TextFileImporter::new("./data/testfile.txt".to_string());
        match importer.init() {
            Ok(_) => {
                let records = importer.next(None);
                if let Ok(records) = records {
                    if let Some(records) = records {
                        for record in records {
                            print_record(&record);
                            check_correct_values(record);
                        }
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_read() {
        let mut importer = TextFileImporter::new("./data/testfile.txt".to_string());
        match importer.init() {
            Ok(_) => {
                let _ = importer.read(|record| {
                    print_record(&record);
                    check_correct_values(record);
                });
            }
            Err(e) => panic!("{}", e),
        }
    }
}
