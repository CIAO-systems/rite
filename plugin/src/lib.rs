use std::collections::HashMap;

use import::Importer;
use libloading::{Library, Symbol};

pub type ImporterCreator =
    unsafe fn(name: &str) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>>;

pub struct Plugin {
    importers: HashMap<String, Box<dyn Importer>>,

    importer_creator: Option<ImporterCreator>,

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
            importer_creator: None,
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
                unsafe { self._lib.get(b"create_importer")? };
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
}
