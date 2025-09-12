use std::{collections::HashMap, sync::Arc};

use log::{debug, error};
use model::helper::get_full_path;
use model::xml;
use moka::sync::Cache;

use super::process::Process;
pub struct Rite {
    rite: xml::Rite,
    processes: Vec<Process>,

    // Make sure, the plugin cache is dropped last
    plugin_cache: Cache<String, Arc<model::plugin::Plugin>>,
}

impl Rite {
    pub fn new(xml_file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let variables = create_variables(xml_file_name);

        Ok(Rite {
            rite: xml::file::create_rite(xml_file_name, &variables)?,
            plugin_cache: Cache::builder().build(),
            processes: Vec::new(),
        })
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for process_desc in &self.rite.processes.processes {
            let mut process = Process::new();
            debug!("Initialize process {}", process_desc.id);
            process.init(self, process_desc)?;
            self.processes.push(process);
            debug!("Process {} initialized", process_desc.id);
        }

        Ok(())
    }

    pub fn get_plugin_desc(&self, plugin_name: &str) -> Option<&xml::plugin::Plugin> {
        self.rite
            .plugins
            .plugins
            .iter()
            .find(|&plugin_desc| plugin_desc.id == plugin_name)
    }

    pub fn load_plugin(
        &self,
        plugin_desc: &xml::plugin::Plugin,
    ) -> Result<Arc<model::plugin::Plugin>, Box<dyn std::error::Error>> {
        if let Some(cached_plugin) = self.plugin_cache.get(&plugin_desc.id) {
            return Ok(cached_plugin);
        }

        // Create the plugin
        let plugin = Arc::new(model::plugin::Plugin::new(
            plugin_desc.path.as_deref(),
            &plugin_desc.name,
        )?);
        self.plugin_cache
            .insert(plugin_desc.id.clone(), plugin.clone());
        Ok(plugin)
    }

    pub fn process(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Start processing...");
        for process in &mut self.processes {
            debug!("Processing {}...", process.id);
            // execute it
            process.run()?;
        }
        debug!("End processing...");
        Ok(())
    }
}

pub fn create_variables(xml_file_name: &str) -> HashMap<String, String> {
    let mut variables: HashMap<String, String> = std::collections::HashMap::new();
    match get_full_path(xml_file_name) {
        Ok(full_path) => {
            //
            if let Some(parent_path) = full_path.parent() {
                if let Some(parent_path) = parent_path.to_str() {
                    variables.insert(
                        String::from(crate::variables::RITE_CONFIG_PATH),
                        String::from(parent_path),
                    );
                }
            }
        }
        Err(e) => error!("Error while getting full path for {}: {}", xml_file_name, e),
    }

    variables
}

#[cfg(test)]
mod tests;