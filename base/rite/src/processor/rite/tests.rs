// use crate::processor::rite::Rite;

// #[test]
// fn test_rite_new() {
//     let rite = Rite::new("../../examples/data/example.xml");
//     assert!(rite.is_ok());

//     let rite = rite.unwrap();
//     let xml = rite.rite;
//     let processes = rite.processes;

//     assert_eq!(processes.len(), 0);
//     assert_eq!(xml.plugins.plugins.len(), 3);
//     assert_eq!(xml.processes.processes.len(), 1);

//     let process = &xml.processes.processes[0];

//     println!("{:#?}", process);
//     assert_eq!(process.id, "text-uppercase-console");
//     assert_eq!(process.importer.plugin, "import_plugin");
//     assert_eq!(process.importer.name, Some("text".to_string()));
// }

// #[test]
// fn test_rite_init() {
//     let dir = std::env::current_dir().unwrap();
//     std::env::set_current_dir(std::path::Path::new("../../examples/")).unwrap();
//     let rite = Rite::new("data/example.xml");
//     assert!(rite.is_ok());
//     let mut rite = rite.unwrap();
//     let result = rite.init();
//     assert!(result.is_ok());
//     std::env::set_current_dir(dir).unwrap();
// }
