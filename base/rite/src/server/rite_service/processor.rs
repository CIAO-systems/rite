use std::{
    fs::{self, remove_dir_all},
    io::Cursor,
    path::PathBuf,
};

use model::BoxedError;
use rite::processor;
use uuid::Uuid;

use crate::proto::rite::v1::ProcessResponse;

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

    pub fn process(&self) -> Result<ProcessResponse, BoxedError> {
        log::info!("Processing {} ...", self.main_config);

        let mut path = PathBuf::from(&self.root_directory);
        path.push(&self.main_config);
        let filename = path.to_str().ok_or::<BoxedError>(format!("").into())?;

        let mut response = ProcessResponse {
            success: false,
            error_message: None,
        };

        let mut rp = processor::rite::Rite::new(filename)?;
        match rp.init() {
            Ok(_) => match rp.process() {
                Ok(_) => {
                    response.success = true;
                    log::info!("Successfully processed")
                }
                Err(e) => handle_error(&mut response, "Error processing:", e),
            },
            Err(e) => handle_error(&mut response, "Error initializing:", e),
        }

        Ok(response)
    }
}

fn handle_error(response: &mut ProcessResponse, prefix: &str, e: BoxedError) {
    let error_message = format!("{prefix} {e}");
    log::error!("{error_message}");
    response.error_message = Some(error_message);
}

impl Drop for ServiceProcessor {
    fn drop(&mut self) {
        if let Err(e) = remove_dir_all(self.root_directory.clone()) {
            log::error!("Error while cleaning up: {}", e);
        }
    }
}
