use log::debug;
use model::record::Record;

use super::rite::Rite;

pub struct Process {
    importer: Option<Box<dyn import::Importer>>,
    transformers: Option<Vec<Box<dyn transform::Transformer>>>,
    exporters: Option<Vec<Box<dyn export::Exporter>>>,
}

impl Process {
    pub fn new() -> Self {
        Process {
            importer: None,
            transformers: None,
            exporters: None,
        }
    }

    /// Load all the importer, transformers and exporters
    pub fn init(
        &mut self,
        rite: &Rite,
        process_desc: &model::xml::process::Process,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // fill importer
        self.fill_importer(rite, process_desc)?;

        // fill transformers
        self.fill_transformers(rite, process_desc)?;

        // fill exporters
        self.fill_exporters(rite, process_desc)?;

        Ok(())
    }

    fn fill_importer(
        &mut self,
        rite: &Rite,
        process_desc: &model::xml::process::Process,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(
            if let Some(plugin_desc) = rite.get_plugin_desc(&process_desc.importer.plugin.as_str())
            {
                debug!("Importer plugin: {:#?}", plugin_desc);

                let plugin = rite.load_plugin(plugin_desc)?;
                let mut importer = plugin.create_importer(process_desc.importer.name.as_deref())?;

                let config = &process_desc.importer.configuration;
                let _ = importer.init(config.clone())?;
                self.importer = Some(importer);
            },
        )
    }

    fn fill_transformers(
        &mut self,
        rite: &Rite,
        process_desc: &model::xml::process::Process,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(transformers_desc) = process_desc
            .transformers
            .as_ref()
            .and_then(|td| td.transformers.as_ref())
        {
            for transformer_desc in transformers_desc {
                if let Some(plugin_desc) = rite.get_plugin_desc(&transformer_desc.plugin.as_str()) {
                    debug!("Transformer plugin: {:#?}", plugin_desc);

                    let plugin = rite.load_plugin(plugin_desc)?;
                    let mut transformer =
                        plugin.create_transformer(transformer_desc.name.as_deref())?;

                    let _ = transformer.init(transformer_desc.configuration.clone())?;

                    self.transformers
                        .get_or_insert_with(Vec::new)
                        .push(transformer);
                }
            }
        }

        Ok(())
    }

    fn fill_exporters(
        &mut self,
        rite: &Rite,
        process_desc: &model::xml::process::Process,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for exporter_desc in &process_desc.exporters.exporters {
            if let Some(plugin_desc) = rite.get_plugin_desc(&exporter_desc.plugin.as_str()) {
                debug!("Exporter plugin: {:#?}", plugin_desc);

                let plugin = rite.load_plugin(plugin_desc)?;
                let mut exporter = plugin.create_exporter(exporter_desc.name.as_deref())?;
                let _ = exporter.init(exporter_desc.configuration.clone())?;
                self.exporters.get_or_insert_with(Vec::new).push(exporter);
            }
        }

        Ok(())
    }

    /// Run the importer, transformers and exporters
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Import data using the importer
        if let Some(ref mut importer) = self.importer {
            let mut records = Vec::new();
            if let Err(e) = importer.read(&mut |record| {
                records.push(Record::copy(record));
            }) {
                log::error!("Error while importing records: {}", e);
            }

            for record in records {
                match self.transform(&record) {
                    Ok(tr) => {
                        if let Some(tr) = tr {
                            if let Err(e) = self.export(&tr) {
                                log::error!("Error while exporting record: {}", e);
                            }
                        }
                    }
                    Err(e) => log::error!("Error while transforming a record: {}", e),
                }
            }
        }

        Ok(())
    }

    fn transform(&self, record: &Record) -> Result<Option<Record>, Box<dyn std::error::Error>> {
        if let Some(ref transformers) = self.transformers {
            let mut transformed_record = Record::copy(record);
            for transformer in transformers {
                transformed_record = transformer.process(&transformed_record)?;
            }
            Ok(Some(transformed_record))
        } else {
            Ok(None)
        }
    }

    /// Exports the record to all exporters
    ///
    fn export(&mut self, record: &Record) -> Result<(), Box<dyn std::error::Error>> {
        // Export to every configured exporter
        if let Some(ref mut exporters) = self.exporters {
            for exporter in exporters {
                exporter.write(record)?;
            }
        }

        Ok(())
    }
}
