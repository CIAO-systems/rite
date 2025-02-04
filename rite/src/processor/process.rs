use exporter::Exporter;
use importer::Importer;
use transformer::Transformer;

pub mod exporter;
pub mod importer;
pub mod transformer;
use super::rite::Rite;
use colored::*;

pub struct Process {
    pub id: String,
    importer: Option<Box<dyn import::Importer>>,
    transformers: Option<Vec<Box<dyn transform::Transformer>>>,
    exporters: Option<Vec<Box<dyn export::Exporter>>>,
}

impl Process {
    pub fn new() -> Self {
        Process {
            id: "".to_string(),
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
        log::debug!("Init process {}", process_desc.id);
        self.id = process_desc.id.clone();
        // fill importer
        self.fill_importer(rite, process_desc)?;
        log::debug!("Filled importer {}", process_desc.id);

        // fill transformers
        self.fill_transformers(rite, process_desc)?;
        log::debug!("Filled transformers {}", process_desc.id);

        // fill exporters
        self.fill_exporters(rite, process_desc)?;
        log::debug!("Filled exporters {}", process_desc.id);

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
                log::debug!("Importer plugin: {:#?}", plugin_desc);

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
                    log::debug!("Transformer plugin: {:#?}", plugin_desc);

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
            log::debug!(
                "Add exporter: plugin({}), name({})",
                exporter_desc.plugin,
                exporter_desc
                    .name
                    .clone()
                    .unwrap_or(String::from("<no name>"))
            );

            if let Some(plugin_desc) = rite.get_plugin_desc(&exporter_desc.plugin.as_str()) {
                log::debug!(
                    "Exporter plugin: id({}), name({}), path({})",
                    plugin_desc.id,
                    plugin_desc.name,
                    plugin_desc.path.clone().unwrap_or("<default>".to_string())
                );

                let plugin = rite.load_plugin(plugin_desc)?;
                let mut exporter = plugin.create_exporter(exporter_desc.name.as_deref())?;
                let _ = exporter.init(exporter_desc.configuration.clone())?;
                self.exporters.get_or_insert_with(Vec::new).push(exporter);
            } else {
                log::error!(
                    "Exporter plugin {} not found",
                    exporter_desc.plugin.yellow().bold()
                );
            }
        }

        Ok(())
    }

    /// Run the importer, transformers and exporters
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (mut importer, transformer, mut exporter) = self.create();

        if let Some(ref mut importer) = importer {
            importer.import(&transformer, &mut exporter)?;
            // FIXME exporter.as_ref().map(|e| e.event(Signal::End));
        }

        Ok(())
    }

    fn create(
        &mut self,
    ) -> (
        Option<Importer<'_>>,
        Option<Transformer<'_>>,
        Option<Exporter<'_>>,
    ) {
        let i = if let Some(ref mut importer) = self.importer {
            log::info!("Create importer");
            Some(Importer::new(importer))
        } else {
            None
        };

        let t = if let Some(ref transformers) = self.transformers {
            log::info!("Create transformers");
            Some(Transformer::new(&transformers))
        } else {
            None
        };

        let e = if let Some(ref mut exporters) = self.exporters {
            log::info!("Create exporters");
            Some(Exporter::new(exporters))
        } else {
            None
        };

        (i, t, e)
    }
}
