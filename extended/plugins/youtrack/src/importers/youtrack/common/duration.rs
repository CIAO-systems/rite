use model::record::Record;
use serde::{Deserialize, Serialize};

use crate::importers::youtrack::factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct DurationValue {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: Option<String>,
    pub minutes: Option<i32>,
    pub presentation: Option<String>,
}

impl From<DurationValue> for Record {
    fn from(value: DurationValue) -> Self {
        factory::serialize_to_record(value)
    }
}

#[cfg(test)]
mod tests {
    use model::record::Record;

    use crate::importers::youtrack::common::duration::DurationValue;

    fn test_value() -> DurationValue {
        let json: &str = r#"{
        "$type": "DurationValue",
        "id": "DV-1",
        "minutes": 15
    }"#;
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn test_from() {
        let dv = test_value();
        let record: Record = dv.into();
        println!("{:?}", record);

        assert!(record.field_by_name("id").is_some());
        assert!(record.field_by_name("minutes").is_some());
        assert!(record.field_by_name("presentation").is_none());
    }
}
