use super::Timestamp;
use chrono::*;
use format::ParseErrorKind;

#[allow(dead_code)]
impl Timestamp {
    /// Parses date times with the format "%Y-%m-%dT%H:%M:%S"
    fn parse_datetime_from(s: &str) -> Result<DateTime<Utc>, ParseError> {
        match s.parse::<DateTime<Utc>>() {
            Ok(dt) => Ok(dt),
            Err(e) => {
                match e.kind() {
                    ParseErrorKind::TooShort => {
                        // Let's try without timezone then
                        let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")?;
                        Ok(Utc.from_utc_datetime(&naive))
                    }
                    _ => Err(e),
                }
            }
        }
    }

    /// Parses the given `s` and returns a [Timestamp] or a [ParseError]
    pub fn parse_from(s: &str) -> Result<Timestamp, ParseError> {
        match Timestamp::parse_datetime_from(s) {
            Ok(dt) => Ok(Timestamp::from_utc(&dt)),
            Err(e) => Err(e),
        }
    }

    /// Converts a UTC time to a local [Timestamp] by adding the offset of the current
    /// local time (see [Local::now()])
    pub fn local_from_utc(dt: &DateTime<Utc>) -> Timestamp {
        Timestamp::from_utc(dt)
    }

    fn from_utc(dt: &DateTime<Utc>) -> Timestamp {
        Timestamp {
            time_utc: Some(prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
            time_zone: iana_time_zone::get_timezone().unwrap_or(String::from("")),
        }
    }

    fn from_datetime_offset(dt: &DateTime<FixedOffset>) -> Timestamp {
        Timestamp::from_utc(&dt.with_timezone(&Utc))
    }

    /// Tries to convert the timestamp to a UTC datetime.
    fn as_utc(&self) -> Option<DateTime<Utc>> {
        if let Some(ref time_utc) = self.time_utc {
            return match Utc.timestamp_opt(time_utc.seconds, time_utc.nanos as u32) {
                LocalResult::Single(result) => Some(result),
                LocalResult::Ambiguous(result, _) => Some(result),
                LocalResult::None => None,
            };
        }
        None
    }

    /// Returns the year of this [Timestamp]
    ///
    /// # Example
    /// ```
    /// let value = "2024-01-01T07:15:37";
    /// let dt = ciao_rs::ciao::common::Timestamp::parse_from(value);
    /// match dt {
    ///    Ok(dt) => {
    ///        println!("The year is {}", dt.year());
    ///    }
    ///    Err(e) => {
    ///        println!("Could not parse date/time: {}", e);
    ///    }
    /// }
    ///
    /// ```
    pub fn year(&self) -> i32 {
        match self.as_utc() {
            Some(dt_utc) => return dt_utc.year(),
            _ => 0,
        }
    }

    /// Returns the month of this [Timestamp]
    pub fn month(&self) -> u32 {
        match self.as_utc() {
            Some(dt_utc) => return dt_utc.month(),
            _ => 0,
        }
    }

    /// Returns the day of this [Timestamp]
    pub fn day(&self) -> u32 {
        match self.as_utc() {
            Some(dt_utc) => return dt_utc.day(),
            _ => 0,
        }
    }

    /// Returns the hour of this [Timestamp]
    pub fn hour(&self) -> u32 {
        match self.as_utc() {
            Some(dt_utc) => return dt_utc.hour(),
            _ => 0,
        }
    }

    /// Returns the minute of this [Timestamp]
    pub fn minute(&self) -> u32 {
        match self.as_utc() {
            Some(dt_utc) => return dt_utc.minute(),
            _ => 0,
        }
    }

    /// Returns the second of this [Timestamp]
    pub fn second(&self) -> u32 {
        match self.as_utc() {
            Some(dt_utc) => return dt_utc.second(),
            _ => 0,
        }
    }
}

#[allow(unused_imports)] // the imports are used, but rust-analyzer thinks they are not
pub(crate) mod tests {
    use std::time::SystemTime;

    use super::Timestamp;
    use chrono::*;

    #[test]
    fn test_from_utc_ok() {
        let dt = Utc::now();
        let timestamp = Timestamp::from_utc(&dt);
        let expected_tz = iana_time_zone::get_timezone().unwrap_or(String::from(""));

        assert_eq!(
            timestamp.time_utc,
            Some(prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            })
        );
        assert_eq!(timestamp.time_zone, expected_tz);
    }

    #[test]
    pub(crate) fn test_from_datetime() {}

    #[test]
    pub(crate) fn test_parse_from_without_zone() {
        let value = "2024-01-01T07:15:37";
        let dt = Timestamp::parse_from(value);
        match dt {
            Ok(dt) => {
                assert_eq!(2024, dt.year());
                assert_eq!(01, dt.month());
                assert_eq!(01, dt.day());
                assert_eq!(7, dt.hour());
                assert_eq!(15, dt.minute());
                assert_eq!(37, dt.second());
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    pub(crate) fn test_parse_from_with_zone_utc() {
        let value = "2024-01-01T07:15:37Z";
        let dt = Timestamp::parse_from(value);
        match dt {
            Ok(dt) => {
                assert_eq!(2024, dt.year());
                assert_eq!(01, dt.month());
                assert_eq!(01, dt.day());
                assert_eq!(7, dt.hour());
                assert_eq!(15, dt.minute());
                assert_eq!(37, dt.second());
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    pub(crate) fn test_parse_from_with_offset_2() {
        let value = "2024-01-01T07:15:37+02:00";
        let dt = Timestamp::parse_from(value);
        match dt {
            Ok(dt) => {
                assert_eq!(2024, dt.year());
                assert_eq!(01, dt.month());
                assert_eq!(01, dt.day());
                assert_eq!(5, dt.hour()); // 7 - 2 = 5
                assert_eq!(15, dt.minute());
                assert_eq!(37, dt.second());
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    pub(crate) fn test_parse_from_invalid() {
        let value = "2024/01/01T07:15:37";
        let dt = Timestamp::parse_from(value);
        assert!(dt.is_err_and(|e| e.kind() == format::ParseErrorKind::Invalid));
    }
}
