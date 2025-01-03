use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dataset {
    pub path: String,
    pub resource: String,
    #[serde(rename = "sub-resource")]
    pub sub_resource: String,
    pub fields: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RiteYoutrackImport {
    pub dataset: Dataset,
}

#[cfg(test)]
mod tests {
    use crate::importers::config::RiteYoutrackImport;
    use serde_xml_rs::from_str;

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        let xml = r#"<rite-youtrack-import>
                <dataset 
                    path="issues" 
                    resource="INTERNAL-32" 
                    sub-resource="timeTracking/workItems" 
                    fields="author(id,name),date,duration(id,minutes,presentation),id,name,type(id,name)"
                />
            </rite-youtrack-import>"#;

        let result: RiteYoutrackImport = from_str(xml)?;
        println!("{:#?}", result);

        assert_eq!("issues", result.dataset.path);
        assert_eq!("INTERNAL-32", result.dataset.resource);
        assert_eq!("timeTracking/workItems", result.dataset.sub_resource);
        assert_eq!(
            "author(id,name),date,duration(id,minutes,presentation),id,name,type(id,name)",
            result.dataset.fields
        );

        Ok(())
    }
}
