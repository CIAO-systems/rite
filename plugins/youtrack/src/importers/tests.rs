use crate::importers::{CFG_TOKEN, CFG_URL};

use super::YouTrackImporter;

#[test]
fn test_check_config() {
    let mut yti = YouTrackImporter::new();
    assert_eq!(yti.check_config(), Some(CFG_URL));
    yti.url = Some("demo-url".to_string());
    assert_eq!(yti.check_config(), Some(CFG_TOKEN));
    yti.token = Some("token".to_string());
    assert_eq!(yti.check_config(), None);
}