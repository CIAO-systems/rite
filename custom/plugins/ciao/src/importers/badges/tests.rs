use ciao_rs::ciao::accounts::badges::Badge;
use model::{
    import::{handlers::ClosureRecordHandler, Importer},
    xml::config::Configuration,
    Initializable,
};

use crate::importers::badges::{handle_badge, Badges};

#[test]
fn test_init() {
    let mut importer = Badges::new();
    let config = Configuration::new();
    let result = importer.init(Some(config));
    assert!(result.is_ok());
}

#[test]
fn test_read() {
    let mut importer = Badges::new();
    let mut handler = ClosureRecordHandler::new(|_| {});
    let config = Configuration::new();
    let result = importer.init(Some(config));
    assert!(result.is_ok());
    let result = importer.read(&mut handler);
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
}

#[test]
fn test_handle_badge() {
    let badge = Badge {
        id: "id".into(),
        user_id: "user_id".into(),
        external_id: "external_id".into(),
        description: Some("description".into()),
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert_eq!(r.field_by_name("id").unwrap().value(), "id".into());
        assert_eq!(
            r.field_by_name("userId").unwrap().value(),
            "user_id".into()
        );
        assert_eq!(
            r.field_by_name("externalId").unwrap().value(),
            "external_id".into()
        );
        assert_eq!(
            r.field_by_name("description").unwrap().value(),
            "description".into()
        );
    });
    let result = handle_badge(&badge, &mut handler);
    assert!(result.is_ok());
}
