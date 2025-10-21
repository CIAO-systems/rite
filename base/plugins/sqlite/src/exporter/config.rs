use model::xml::common::Table;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-sqlite-export")]
pub struct RiteSQLiteExport {
    pub filename: String,
    pub table: Table,
}

#[cfg(test)]
mod tests {
    use crate::exporter::config::RiteSQLiteExport;

    #[test]
    fn test_xml() {
        let xml = r#"
        <rite-sqlite-export>
            <filename>/tmp/demo.db</filename>
            <table name="customer" uniqueFields="id"/>
        </rite-sqlite-export>
        "#;

        let config: RiteSQLiteExport = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!(config.filename, "/tmp/demo.db");
        assert_eq!(config.table.name, "customer");
    }
}
