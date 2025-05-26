use model::Initializable;

const CFG_FILENAME: &str = "filename";
const CFG_EXPORT_OVERWRITE: &str = "overwrite";
const CFG_DELIMITER: &str = "delimiter";

#[derive(Debug)]
pub struct CSV {
    filename: Option<String>,
    delimiter: Option<u8>,
    export_header_written: bool,
    export_override: bool,
}

impl CSV {
    pub(crate) fn new() -> Self {
        CSV {
            filename: None,
            delimiter: None,
            export_header_written: false,
            export_override: false,
        }
    }

    fn push_delimiter(&self, s: &mut String) {
        s.push(self.delimiter.unwrap_or(b',') as char);
    }
}

impl Initializable for CSV {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.filename = config.get(CFG_FILENAME);
            self.export_override = match config.get(CFG_EXPORT_OVERWRITE) {
                Some(value) => value.parse::<bool>()?,
                None => false,
            };
            self.delimiter = config
                .get(CFG_DELIMITER)
                .and_then(|s| s.as_bytes().first().copied());
            log::debug!("CSV: {:?}", self);
        }

        Ok(())
    }
}

mod exporter;
mod importer;

#[cfg(test)]
mod tests {
    use model::{xml::config::Configuration, BoxedError};

    use super::*;

    #[test]
    fn test_push_delimiter_with_some_delimiter() -> Result<(), BoxedError> {
        let mut csv = CSV::new();
        let mut config = Configuration::new();
        config.insert_str(CFG_DELIMITER, ";");
        csv.init(Some(config))?;

        let mut s = String::new();
        s.push_str("field1");
        csv.push_delimiter(&mut s);
        s.push_str("field2");

        assert_eq!("field1;field2", s, "Delimiter should be ;");

        Ok(())
    }

    #[test]
    fn test_push_delimiter_without_delimiter() -> Result<(), BoxedError> {
        let csv = CSV::new();
        let mut s = String::new();
        s.push_str("field1");
        csv.push_delimiter(&mut s);
        s.push_str("field2");

        assert_eq!("field1,field2", s, "Delimiter should be ,");

        Ok(())
    }
}
