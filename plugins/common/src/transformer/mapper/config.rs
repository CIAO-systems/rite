use std::{collections::HashMap, sync::OnceLock};

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Field<'a> {
    name: Name,
    #[serde(rename = "type")]
    field_type: Type,
    values: Values<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Name {
    source: String,
    target: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Type {
    source: String,
    target: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Values<'a> {
    value: Vec<Value>,

    #[serde(skip)]
    map: OnceLock<HashMap<&'a String, &'a Value>>,
}

impl<'a> Values<'a> {
    #[allow(unused)]
    pub fn get(&'a self, key: String) -> Option<&'a Value> {
        let map = self.map.get_or_init(|| {
            let mut map = HashMap::new();
            for v in &self.value {
                map.insert(&v.source, v);
            }
            map
        });

        map.get(&key).copied()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Value {
    source: String,
    target: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Mapper<'a> {
    #[serde(rename = "field")]
    fields: Vec<Field<'a>>,
}

#[allow(dead_code)]
fn read_from_xml(xml_data: &str) -> Result<Mapper, serde_xml_rs::Error> {
    from_str(xml_data)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_xml_parsing1() {
        let parsed = read_from_xml(XML_DATA1).expect("Failed to parse XML");

        assert_eq!(parsed.fields.len(), 2);

        // Check the first field
        let field1 = &parsed.fields[0];
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
        assert_eq!(field1.values.value.len(), 3);

        assert_eq!(
            field1
                .values
                .get("1ced5054-b109-4b9b-aab1-72e1ace3ef54".to_string()),
            Some(&Value {
                source: String::from("1ced5054-b109-4b9b-aab1-72e1ace3ef54"),
                target: String::from("2"),
            })
        );

        assert_eq!(
            field1.values.value.get(0),
            Some(&Value {
                source: "".to_string(),
                target: "0".to_string()
            })
        );
        assert_eq!(
            field1.values.value.get(1),
            Some(&Value {
                source: "b691be93-7272-4089-a575-9f3a3d951c2e".to_string(),
                target: "1".to_string()
            })
        );

        // Check the second field
        let field2 = &parsed.fields[1];
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
        assert_eq!(field2.values.value.len(), 4);
        assert_eq!(
            field2.values.value.get(0),
            Some(&Value {
                source: "Samuel Clemens".to_string(),
                target: "Mark Twain".to_string()
            })
        );
        assert_eq!(
            field2.values.value.get(1),
            Some(&Value {
                source: "Farrokh Bulsara".to_string(),
                target: "Freddy Mercury".to_string()
            })
        );

        assert_eq!(
            field2.values.get("Farrokh Bulsara".to_string()),
            Some(&Value {
                source: "Farrokh Bulsara".to_string(),
                target: "Freddy Mercury".to_string()
            })
        );

        assert_eq!(
            field2.values.get("Cassius Clay".to_string()),
            Some(&Value {
                source: "Cassius Clay".to_string(),
                target: "Muhammad Ali".to_string()
            })
        );
    }

    #[test]
    fn test_xml_parsing2() {
        let parsed = read_from_xml(XML_DATA2).expect("Failed to parse XML");
        assert_eq!(
            parsed.fields[0].values.get("vs1".to_string()),
            Some(&Value {
                source: "vs1".to_string(),
                target: "vt1".to_string()
            })
        );

        assert_eq!(
            parsed.fields[0].values.get("unknown value".to_string()),
            None
        );
    }
}
