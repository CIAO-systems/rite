use import::Importer;
use model::{record::Record, value::Value};

use super::TextFileImporter;

static TEST_DATA: &str = "../../data/testfile.txt";

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
    let mut importer = TextFileImporter::new(TEST_DATA.to_string());
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
    let mut importer = TextFileImporter::new(TEST_DATA.to_string());
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
    let mut importer = TextFileImporter::new(TEST_DATA.to_string());
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
    let mut importer = TextFileImporter::new(TEST_DATA.to_string());
    match importer.init() {
        Ok(_) => {
            let _ = importer.read(&mut |record| {
                print_record(&record);
                check_correct_values(record);
            });
        }
        Err(e) => panic!("{}", e),
    }
}
