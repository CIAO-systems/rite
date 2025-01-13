use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dataset {
    pub path: String,
    pub resource: Option<String>,
    pub query: Option<String>,
    #[serde(rename = "sub-resource")]
    pub sub_resource: Option<String>,
    pub fields: Fields,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fields {
    #[serde(rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    //    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RiteYoutrackImport {
    pub dataset: Dataset,
}

impl Fields {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for field in &self.fields {
            if !field.id.is_empty() {
                if !result.is_empty() {
                    result.push(',');
                }
                result.push_str(&field.id);
            }
        }
        result
    }

    pub fn contains(&self, id: &str) -> bool {
        self.fields.iter().any(|element| element.id.starts_with(id))
    }
}

#[cfg(test)]
mod tests {
    use crate::importers::generic::config::RiteYoutrackImport;
    use serde_xml_rs::from_str;

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        let xml = r#"
    <rite-youtrack-import>
        <dataset path="issues"> 
            <query>for: me</query>
            <fields>
                <field id="id" />
                <field id="idReadable" />
                <field id="numberInProject" />
                <field id="project(id,name)" />
                <field id="commentsCount" />
                <field id="created" />
                <field id="summary" />
                <field id="" />
                <field id="" />
                <field id="" />
            </fields>
        </dataset>
    </rite-youtrack-import>
    "#;

        let result: RiteYoutrackImport = from_str(xml)?;
        println!("{:#?}", result);

        assert_eq!("issues", result.dataset.path);
        assert_eq!(None, result.dataset.resource);
        assert_eq!(Some("for: me".into()), result.dataset.query);
        assert_eq!(None, result.dataset.sub_resource);

        assert_eq!(10, result.dataset.fields.fields.len());
        assert_eq!(result.dataset.fields.fields[0].id, "id");
        assert_eq!(result.dataset.fields.fields[1].id, "idReadable");
        assert_eq!(result.dataset.fields.fields[2].id, "numberInProject");
        assert_eq!(result.dataset.fields.fields[3].id, "project(id,name)");
        assert_eq!(result.dataset.fields.fields[4].id, "commentsCount");
        assert_eq!(result.dataset.fields.fields[5].id, "created");
        assert_eq!(result.dataset.fields.fields[6].id, "summary");
        assert_eq!(result.dataset.fields.fields[7].id, "");
        assert_eq!(result.dataset.fields.fields[8].id, "");
        assert_eq!(result.dataset.fields.fields[9].id, "");

        Ok(())
    }
}
