use std::{
    fs::{self, remove_dir_all},
    io::Cursor,
    path::PathBuf,
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

    pub fn process(&self) -> Result<bool, BoxedError> {
        println!("Process {}", self.main_config);
        // TODO implement
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
