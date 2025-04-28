use serde::{Deserialize, Serialize};

pub mod mapper;
pub mod pattern;
pub mod values;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    pub name: Name,
    #[serde(rename = "type")]
    pub field_type: Type,
    pub patterns: Option<pattern::Patterns>,
    pub values: Option<values::Values>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Name {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Type {
    pub source: String,
    pub target: String,
}

#[cfg(test)]
mod tests {
    use crate::transformer::mapper::config::values::Value;
    use serde_xml_rs::from_str;

    use super::{mapper::Mapper, *};

    static XML_DATA1: &str = r#"
    <mapper>
        <field>
            <name source="timetype" target="account" />
            <type source="uuid" target="int" />
            <values>
                <value source="" target="0" />
                <value source="b691be93-7272-4089-a575-9f3a3d951c2e" target="1" />
                <value source="1ced5054-b109-4b9b-aab1-72e1ace3ef54" target="2" />
            </values>
        </field>
        <field>
            <name source="name" target="newname" />
            <type source="string" target="string" />
            <values>
                <value source="Samuel Clemens" target="Mark Twain" />
                <value source="Farrokh Bulsara" target="Freddy Mercury" />
                <value source="Norma Jean Mortenson" target="Marilyn Monroe" />
                <value source="Cassius Clay" target="Muhammad Ali" />
            </values>
        </field>
    </mapper>
    "#;

    static XML_DATA2: &str = r#"
    <mapper>
        <field>
            <name source="ns1" target="nt1" />
            <type source="ts1" target="tt1" />
            <values source="wtf">
                <value source="vs1" target="vt1" />
            </values>
        </field>
    </mapper>
    "#;

    #[allow(dead_code)]
    fn read_from_xml(xml_data: &str) -> Result<Mapper, serde_xml_rs::Error> {
        from_str(xml_data)
    }

    #[test]
    fn test_xml_parsing1() {
        let parsed = read_from_xml(XML_DATA1).expect("Failed to parse XML");

        // Check the first field
        let field1 = &parsed.get("key".to_string());
        assert!(field1.is_none());
        let field1 = &parsed.get("timetype".to_string()).unwrap();
        assert_eq!(
            field1.name,
            Name {
                source: "timetype".to_string(),
                target: "account".to_string()
            }
        );
        assert_eq!(
            field1.field_type,
            Type {
                source: "uuid".to_string(),
                target: "int".to_string()
            }
        );
        assert_eq!(field1.values.clone().unwrap().value.len(), 3);

        assert_eq!(
            field1
                .values
                .clone()
                .unwrap()
                .get("1ced5054-b109-4b9b-aab1-72e1ace3ef54".to_string()),
            Some(Value {
                source: String::from("1ced5054-b109-4b9b-aab1-72e1ace3ef54"),
                target: String::from("2"),
            })
        );

        assert_eq!(
            field1.values.clone().unwrap().value.get(0),
            Some(&Value {
                source: "".to_string(),
                target: "0".to_string()
            })
        );
        assert_eq!(
            field1.values.clone().unwrap().value.get(1),
            Some(&Value {
                source: "b691be93-7272-4089-a575-9f3a3d951c2e".to_string(),
                target: "1".to_string()
            })
        );

        // Check the second field
        let field2 = &parsed.get("name".to_string()).unwrap();
        assert_eq!(
            field2.name,
            Name {
                source: "name".to_string(),
                target: "newname".to_string()
            }
        );
        assert_eq!(
            field2.field_type,
            Type {
                source: "string".to_string(),
                target: "string".to_string()
            }
        );
        assert_eq!(field2.values.clone().unwrap().value.len(), 4);
        assert_eq!(
            field2.values.clone().unwrap().value.get(0),
            Some(&Value {
                source: "Samuel Clemens".to_string(),
                target: "Mark Twain".to_string()
            })
        );
        assert_eq!(
            field2.values.clone().unwrap().value.get(1),
            Some(&Value {
                source: "Farrokh Bulsara".to_string(),
                target: "Freddy Mercury".to_string()
            })
        );

        assert_eq!(
            field2
                .values
                .clone()
                .unwrap()
                .get("Farrokh Bulsara".to_string()),
            Some(Value {
                source: "Farrokh Bulsara".to_string(),
                target: "Freddy Mercury".to_string()
            })
        );

        assert_eq!(
            field2
                .values
                .clone()
                .unwrap()
                .get("Cassius Clay".to_string()),
            Some(Value {
                source: "Cassius Clay".to_string(),
                target: "Muhammad Ali".to_string()
            })
        );
    }

    #[test]
    fn test_xml_parsing2() {
        let parsed = read_from_xml(XML_DATA2).expect("Failed to parse XML");

        let field = parsed.get("ns1".to_string()).unwrap();
        assert_eq!(
            field.values.clone().unwrap().get("vs1".to_string()),
            Some(Value {
                source: "vs1".to_string(),
                target: "vt1".to_string()
            })
        );

        assert_eq!(field.values.unwrap().get("unknown value".to_string()), None);
    }
}
