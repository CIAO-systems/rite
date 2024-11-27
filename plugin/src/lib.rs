use std::collections::HashMap;

use export::Exporter;
use import::Importer;
use libloading::{Library, Symbol};

const CREATE_EXPORTER: &[u8] = b"create_exporter";
const CREATE_IMPORTER: &[u8] = b"create_importer";

pub type ImporterCreator =
    unsafe fn(name: &str) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>>;
pub type ExporterCreator =
    unsafe fn(name: &str) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>>;

pub struct Plugin {
    importers: HashMap<String, Box<dyn Importer>>,
    exporters: HashMap<String, Box<dyn Exporter>>,

    importer_creator: Option<ImporterCreator>,
    exporter_creator: Option<ExporterCreator>,

    // this must be last, so it get dropped last
    _lib: Library,
}

impl Plugin {
    /// Creates an instance of the [Plugin] struct.
    ///
    /// `path` is the fully qualified directory name, where the dynamic library is
    /// located
    /// `name` is a platorm agnostic name of the library (without prefix `lib` and
    /// without extension)
    pub fn new(path: &str, name: &str) -> Result<Plugin, Box<dyn std::error::Error>> {
        //
        #[cfg(target_os = "linux")]
        let os_lib_name = format!("lib{name}.so");
        #[cfg(target_os = "macos")]
        let os_lib_name = format!("lib{name}.dylib");
        #[cfg(target_os = "windows")]
        let os_lib_name = format!("{name}.dll");

        let lib_path = format!("{path}/{os_lib_name}");
        let _lib = unsafe { Library::new(lib_path)? };

        Ok(Self {
            importers: HashMap::new(),
            exporters: HashMap::new(),
            importer_creator: None,
            exporter_creator: None,
            _lib,
        })
    }

    pub fn create_importer(
        &mut self,
        importer_name: &str,
    ) -> Result<&mut Box<dyn Importer>, Box<dyn std::error::Error>> {
        // Create the method lazily
        if let None = self.importer_creator {
            let importer_creator: Symbol<ImporterCreator> =
                unsafe { self._lib.get(CREATE_IMPORTER)? };
            self.importer_creator = Some(*importer_creator);
        }

        // Only, if we have a creator function
        if let Some(creator) = self.importer_creator {
            // If the element is already in our map, return it,
            // otherwise create, insert and return it.
            self.importers
                .entry(importer_name.to_string())
                .or_insert_with(|| unsafe {
                    match creator(importer_name) {
                        Ok(v) => v,
                        Err(e) => panic!("Creating importer failed: {e}"),
                    }
                });
        }

        if let Some(importer) = self.importers.get_mut(importer_name) {
            // we have an instance of this importer
            return Ok(importer);
        }

        Err("Cannot create importer".into())
    }

    pub fn create_exporter(
        &mut self,
        exporter_name: &str,
    ) -> Result<&mut Box<dyn Exporter>, Box<dyn std::error::Error>> {
        // Create the method lazily
        if let None = self.exporter_creator {
            let exporter_creator: Symbol<ExporterCreator> =
                unsafe { self._lib.get(CREATE_EXPORTER)? };
            self.exporter_creator = Some(*exporter_creator);
        }

        // Only, if we have a creator function
        if let Some(creator) = self.exporter_creator {
            // If the element is already in our map, return it,
            // otherwise create, insert and return it.
            self.exporters
                .entry(exporter_name.to_string())
                .or_insert_with(|| unsafe {
                    match creator(exporter_name) {
                        Ok(v) => v,
                        Err(e) => panic!("Creating exporter failed: {e}"),
                    }
                });
        }

        if let Some(exporter) = self.exporters.get_mut(exporter_name) {
            // we have an instance of this exporter
            return Ok(exporter);
        }

        Err("Cannot create exporter".into())
    }
}
