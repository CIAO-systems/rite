use std::{
    fs::{self, remove_dir_all},
    io::Cursor,
    path::{Path, PathBuf},
};

use model::BoxedError;
use uuid::Uuid;

pub struct ServiceProcessor {
    main_config: String,
    root_directory: PathBuf,
}

impl ServiceProcessor {
    pub fn new(zipped_configs: &Vec<u8>, main_config: String) -> Result<Self, BoxedError> {
        // create unique temporary directory
        let session_id = Uuid::new_v4();
        let root_directory = std::env::temp_dir().join(format!("rite-session-{}", session_id));
        fs::create_dir(&root_directory)?;

        // extract all files from the zipped configuration
        let reader = Cursor::new(zipped_configs);
        let mut archive = zip::ZipArchive::new(reader)?;
        archive.extract(&root_directory)?;

        Ok(Self {
            root_directory,
            main_config,
        })
    }

    fn list_dir<P: AsRef<Path>>(path: P, indent: usize) -> std::io::Result<()> {
        let entries = fs::read_dir(&path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let indent_str = "  ".repeat(indent);

            if path.is_dir() {
                println!(
                    "{}📁 {}",
                    indent_str,
                    path.file_name().unwrap().to_string_lossy()
                );
                // Recursive call
                ServiceProcessor::list_dir(&path, indent + 1)?;
            } else if path.is_file() {
                println!(
                    "{}📄 {}",
                    indent_str,
                    path.file_name().unwrap().to_string_lossy()
                );
            }
        }

        Ok(())
    }

    pub fn process(&self) -> Result<bool, BoxedError> {
        println!("Processing {} ...", self.main_config);
        // TODO implement
        ServiceProcessor::list_dir(&self.root_directory, 0)?;
        Ok(true)
    }
}

impl Drop for ServiceProcessor {
    fn drop(&mut self) {
        if let Err(e) = remove_dir_all(self.root_directory.clone()) {
            log::error!("Error while cleaning up: {}", e);
        }
    }
}
