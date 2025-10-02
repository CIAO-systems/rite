use std::path::{Path, PathBuf};

use rite::processor::rite::Rite;

struct TemporaryChangeDirectory {
    current_dir: PathBuf,
}

impl TemporaryChangeDirectory {
    pub fn new(new_dir: &str) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_default();
        let _ = std::env::set_current_dir(Path::new(new_dir)).unwrap_or_default();

        println!(
            "new after {}",
            std::env::current_dir().unwrap_or_default().display()
        );

        TemporaryChangeDirectory { current_dir }
    }
}

impl Drop for TemporaryChangeDirectory {
    fn drop(&mut self) {
        println!(
            "drop before {}",
            std::env::current_dir().unwrap_or_default().display()
        );
        let _ = std::env::set_current_dir(self.current_dir.clone()).unwrap_or_default();
        println!(
            "drop after {}",
            std::env::current_dir().unwrap_or_default().display()
        );
    }
}

#[test]
fn test_rite_new_and_init() {
    let _tcd = TemporaryChangeDirectory::new("../../examples");
    let rite = Rite::new("data/example.xml");
    assert!(rite.is_ok());

    match rite {
        Ok(mut rite) => {
            let result = rite.init(); // this will fail, when there are no dynamic libraries in the path
            match result {
                Ok(_) => {
                    assert_eq!(rite.processes.len(), 1);
                    let xml = rite.rite;
                    assert_eq!(xml.plugins.plugins.len(), 3);
                    assert_eq!(xml.processes.processes.len(), 1);

                    let process = &xml.processes.processes[0];

                    // println!("{:#?}", process);
                    assert_eq!(process.id, "text-uppercase-console");
                    assert_eq!(process.importer.plugin, "import_plugin");
                    assert_eq!(process.importer.name, Some("text".to_string()));
                }
                Err(e) => {
                    eprintln!("{e}");
                    assert_eq!(rite.processes.len(), 0);
                }
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}
