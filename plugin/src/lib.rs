use std::collections::HashMap;

use export::Exporter;
use import::Importer;
use libloading::{Library, Symbol};
use transform::Transformer;

const CREATE_EXPORTER: &[u8] = b"create_exporter";
const CREATE_IMPORTER: &[u8] = b"create_importer";
const CREATE_TRANSFORMER: &[u8] = b"create_transformer";

pub type ImporterCreator =
    unsafe fn(name: &str) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>>;
pub type ExporterCreator =
    unsafe fn(name: &str) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>>;
pub type TransformerCreator =
    unsafe fn(name: &str) -> Result<Box<dyn Transformer>, Box<dyn std::error::Error>>;

pub struct Plugin {
    importers: HashMap<String, Box<dyn Importer>>,
    exporters: HashMap<String, Box<dyn Exporter>>,
    transformers: HashMap<String, Box<dyn Transformer>>,

    importer_creator: Option<ImporterCreator>,
    exporter_creator: Option<ExporterCreator>,
    transformer_creator: Option<TransformerCreator>,

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
        // check the OS we're running on
        let os_lib_name = match std::env::consts::OS {
            "linux" => format!("lib{name}.so"),
            "macos" => format!("lib{name}.dylib"),
            "windows" => format!("{name}.dll"),
            _ => return Err("Unsupported operating system".into()),
        };

        let lib_path = format!("{path}/{os_lib_name}");
        let _lib = unsafe { Library::new(lib_path)? };

        Ok(Self {
            importers: HashMap::new(),
            exporters: HashMap::new(),
            transformers: HashMap::new(),
            importer_creator: None,
            exporter_creator: None,
            transformer_creator: None,
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
                        Ok(importer) => importer,
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

    pub fn create_transformer(
        &mut self,
        transformer_name: &str,
    ) -> Result<&mut Box<dyn Transformer>, Box<dyn std::error::Error>> {
        // Create the method lazily
        if let None = self.transformer_creator {
            let transformer_creator: Symbol<TransformerCreator> =
                unsafe { self._lib.get(CREATE_TRANSFORMER)? };
            self.transformer_creator = Some(*transformer_creator);
        }

        // Only, if we have a creator function
        if let Some(creator) = self.transformer_creator {
            // If the element is already in our map, return it,
            // otherwise create, insert and return it.
            self.transformers
                .entry(transformer_name.to_string())
                .or_insert_with(|| unsafe {
                    match creator(transformer_name) {
                        Ok(v) => v,
                        Err(e) => panic!("Creating transformer failed: {e}"),
                    }
                });
        }

        if let Some(transformer) = self.transformers.get_mut(transformer_name) {
            // we have an instance of this transformer
            return Ok(transformer);
        }

        Err("Cannot create transformer".into())
    }
}
