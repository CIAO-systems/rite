use model::{record::Record, xml};
pub struct Rite {
    rite: xml::Rite,
}

impl Rite {
    pub fn new(xml_file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let rite_processor = Rite {
            rite: xml::file::create_rite(xml_file_name)?,
        };

        Ok(rite_processor)
    }

    fn get_plugin_desc(&self, plugin_name: &str) -> Option<&xml::Plugin> {
        self.rite
            .plugins
            .plugins
            .iter()
            .find(|&plugin_desc| plugin_desc.id == plugin_name)
    }

    pub fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        for process in &self.rite.processes.processes {
            println!("Run process '{}'", process.id);
            // Import data using the importer
            self.import(&process)?;
        }
        Ok(())
    }

    fn import(&self, process: &xml::Process) -> Result<(), Box<dyn std::error::Error>> {
        // Import data using the importer
        if let Some(plugin_desc) = self.get_plugin_desc(&process.importer.plugin.as_str()) {
            println!("Importer plugin: {:#?}", plugin_desc);

            let mut importer_plugin = plugin::Plugin::new(&plugin_desc.path, &plugin_desc.name)?;
            let importer = importer_plugin.create_importer(&process.importer.name)?;

            let config = &process.importer.configuration;
            let _ = importer.init(config.clone())?;

            let _ = importer.read(&mut |record| {
                // transform
                let transformed_record = match self.transform(&record, &process) {
                    Ok(record) => {
                        println!("Transformed record: {:#?}", record);
                        record
                    }
                    Err(e) => panic!("Error transforming: {e}"),
                };

                // export
                if let Some(transformed_record) = transformed_record {
                    let _ = self.export(&transformed_record, &process);
                }
            });
        }
        Ok(())
    }

    fn transform(
        &self,
        record: &Record,
        process: &xml::Process,
    ) -> Result<Option<Record>, Box<dyn std::error::Error>> {
        let mut transformed_record = Record::copy(record);
        for transformer_desc in &process.transformers.transformers {
            if let Some(plugin_desc) = self.get_plugin_desc(&transformer_desc.plugin.as_str()) {
                println!(
                    "Transformer plugin ({}): {:#?}",
                    transformer_desc.name, plugin_desc
                );

                let mut transformer_plugin =
                    plugin::Plugin::new(&plugin_desc.path, &plugin_desc.name)?;
                let transformer = transformer_plugin.create_transformer(&transformer_desc.name)?;

                let config = &transformer_desc.configuration;
                let _ = transformer.init(config.clone())?;
                transformed_record = transformer.process(&transformed_record)?;
            }
        }

        Ok(Some(transformed_record))
    }

    fn export(
        &self,
        record: &Record,
        process: &xml::Process,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for exporter_desc in &process.exporters.exporters {
            if let Some(plugin_desc) = self.get_plugin_desc(&exporter_desc.plugin.as_str()) {
                println!(
                    "Exporter plugin ({}): {:#?}",
                    exporter_desc.name, plugin_desc
                );

                let mut exporter_plugin =
                    plugin::Plugin::new(&plugin_desc.path, &plugin_desc.name)?;
                let exporter = exporter_plugin.create_exporter(&exporter_desc.name)?;

                let config = &exporter_desc.configuration;
                let _ = exporter.init(config.clone())?;

                exporter.write(record)?;
            }
        }

        Ok(())
    }
}
