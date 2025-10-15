use model::{xml::file::load_and_substitute_from_env, Initializable};

use crate::exporter::{config, MariaDBExporter};

impl Initializable for MariaDBExporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let mariadb: config::RiteMariaDBExport =
                            match serde_xml_rs::from_str(&xml_contents) {
                                Ok(x) => x,
                                Err(e) => {
                                    return Err(format!(
                                        "Cannot parse contents from {}: {}",
                                        xml, e
                                    )
                                    .into());
                                }
                            };
                        self.mariadb = Some(mariadb);

                        // Connect here already, because write is called from outside
                        self.connect()?;

                        // Create table, if necessary
                        if self.needs_creating() {
                            self.create()?;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error while loading {}: {}", xml, e);
                        return Err(e.into());
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use model::{xml::config::Configuration, Initializable};

    use crate::exporter::MariaDBExporter;

    #[test]
    fn test_init() {
        let mut exporter = MariaDBExporter::new();
        let config = Configuration::with_xml("../../data/test/mariadb/unit-test-export.xml");
        let result = exporter.init(Some(config));
        assert!(result.is_err());
        let e = result.err().unwrap().to_string();
        assert!(e.contains("unknown:73"));
    }
}