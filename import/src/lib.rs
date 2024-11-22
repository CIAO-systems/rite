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

pub mod builtin {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Seek},
        path::Path,
    };

    use model::{field::Field, record::Record};

    use crate::Importer;

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
                    println!("{:?}", line);
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

        fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            if let Some(reader) = self.reader.as_mut() {
                let _ = reader.seek(std::io::SeekFrom::Start(0))?;
                self.next_line = 0;
            }
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
    fn test_next_all() {
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
    fn test_next_first_three() {
        let mut importer = TextFileImporter::new("./data/testfile.txt".to_string());
        match importer.init() {
            Ok(_) => {
                println!("Read first 3....");
                let records = importer.next(Some(3)); // Here we only want the first 3 records
                if let Ok(records) = records {
                    if let Some(records) = records {
                        assert_eq!(3, records.len());
                        for record in records {
                            print_record(&record);
                            check_correct_values(record);
                        }
                    }
                }

                println!("Read next 3....");
                // read the next three records
                let records = importer.next(Some(3));
                if let Ok(records) = records {
                    if let Some(records) = records {
                        // Since the file has only 5 lines, len must be 2
                        assert_eq!(2, records.len());
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
    fn test_next_first_three_with_reset() {
        let mut importer = TextFileImporter::new("./data/testfile.txt".to_string());
        match importer.init() {
            Ok(_) => {
                println!("Read first 3....");
                let records = importer.next(Some(3)); // Here we only want the first 3 records
                if let Ok(records) = records {
                    if let Some(records) = records {
                        assert_eq!(3, records.len());
                        for record in records {
                            print_record(&record);
                            check_correct_values(record);
                        }
                    }
                }

                println!("Read first 3 again....");
                match importer.reset() {
                    Ok(_) => {
                        // read the first three records again
                        let records = importer.next(Some(3));
                        if let Ok(records) = records {
                            if let Some(records) = records {
                                // Since we resetted, it should be 3
                                assert_eq!(3, records.len());
                                for record in records {
                                    print_record(&record);
                                    check_correct_values(record);
                                }
                            }
                        }
                    }
                    Err(e) => panic!("{e}"),
                };
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
