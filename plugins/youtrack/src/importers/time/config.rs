use chrono::{Datelike, NaiveDate};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct RiteYoutrackImportTime {
    pub time_tracking: TimeTracking,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TimeTracking {
    #[serde(with = "optional_date_format")]
    pub start_date: Option<NaiveDate>,
    #[serde(with = "optional_date_format")]
    pub end_date: Option<NaiveDate>,
}

impl RiteYoutrackImportTime {
    pub fn from(xml_file: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(xml_file)?;

        match serde_xml_rs::from_str(&content) {
            Ok(o) => Ok(o),
            Err(e) => Err(e.into()),
        }
    }
}

impl TimeTracking {
    fn format_date_opt(date: Option<NaiveDate>) -> Option<String> {
        if let Some(date) = date {
            Some(
                format!(
                    "{:04}-{:02}-{:02}",
                    date.year(),
                    date.month0() + 1,
                    date.day0() + 1
                )
                .to_string(),
            )
        } else {
            None
        }
    }

    pub fn start_date_as_param(&self) -> Option<String> {
        TimeTracking::format_date_opt(self.start_date)
    }

    pub fn end_date_as_param(&self) -> Option<String> {
        TimeTracking::format_date_opt(self.end_date)
    }
}

mod optional_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_test_file() {
        match RiteYoutrackImportTime::from(&"../../data/youtrack/tests/time.xml".to_string()) {
            Ok(cfg) => {
                assert_eq!(
                    cfg.time_tracking.start_date,
                    Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
                );
                assert_eq!(cfg.time_tracking.end_date, None);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_deserialize_with_empty_end_date() {
        let xml = r#"
                <rite-youtrack-import-time>
                    <time-tracking
                        start-date="2025-01-01"
                        end-date=""
                    />
                </rite-youtrack-import-time>"#;

        let result: RiteYoutrackImportTime = serde_xml_rs::from_str(xml).unwrap();

        assert_eq!(
            result.time_tracking.start_date,
            Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        );
        assert_eq!(result.time_tracking.end_date, None);
    }

    #[test]
    fn test_deserialize_with_both_dates() {
        let xml = r#"
                <rite-youtrack-import-time>
                    <time-tracking
                        start-date="2025-01-01"
                        end-date="2025-12-31"
                    />
                </rite-youtrack-import-time>"#;

        let result: RiteYoutrackImportTime = serde_xml_rs::from_str(xml).unwrap();

        assert_eq!(
            result.time_tracking.start_date,
            Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        );
        assert_eq!(
            result.time_tracking.end_date,
            Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
        );
    }

    #[test]
    fn test_invalid_date_format() {
        let xml = r#"
                <rite-youtrack-import-time>
                    <time-tracking
                        start-date="2025/01/01"
                        end-date=""
                    />
                </rite-youtrack-import-time>"#;

        let result: Result<RiteYoutrackImportTime, _> = serde_xml_rs::from_str(xml);
        assert!(result.is_err());
    }
}
