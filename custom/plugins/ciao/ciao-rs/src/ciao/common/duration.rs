pub const SECONDS_PER_HOUR: f64 = 60.0 * 60.0;
pub const SECONDS_PER_DAY: f64 = 24.0 * SECONDS_PER_HOUR;

/// Converts a [prost_types::Duration] to hours
pub fn to_hours(duration: prost_types::Duration) -> f64 {
    duration.seconds as f64 / SECONDS_PER_HOUR
}

/// Converts a [prost_types::Duration] to days
pub fn to_days(duration: prost_types::Duration) -> f64 {
    duration.seconds as f64 / SECONDS_PER_DAY
}

/// Subtracts a [prost_types::Duration] from a a [prost_types::Timestamp]
pub fn timestamp_minus(
    timestamp: &prost_types::Timestamp,
    duration: &prost_types::Duration,
) -> prost_types::Timestamp {
    let mut seconds = timestamp.seconds - duration.seconds;
    let mut nanos = timestamp.nanos - duration.nanos;

    // If the nanos overflow, we have to remove a second, and add the nanos
    if nanos < 0 {
        seconds -= 1;
        nanos += 1_000_000_000;
    }

    prost_types::Timestamp { seconds, nanos }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_days() {
        let duration = prost_types::Duration {
            seconds: (SECONDS_PER_DAY * 2.0 + SECONDS_PER_DAY * 0.5) as i64,
            nanos: 0,
        };

        assert_eq!(to_days(duration), 2.5);
    }

    #[test]
    fn test_to_hours() {
        let duration = prost_types::Duration {
            seconds: (SECONDS_PER_DAY * 2.0 + SECONDS_PER_DAY * 0.5) as i64,
            nanos: 0,
        };

        assert_eq!(to_hours(duration), 60.0);
    }

    #[test]
    fn test_timestamp_minus() {
        let timestamp = prost_types::Timestamp {
            seconds: 1000,
            nanos: 0,
        };
        let duration = prost_types::Duration {
            seconds: 100,
            nanos: 0,
        };

        assert_eq!(
            timestamp_minus(&timestamp, &duration),
            prost_types::Timestamp {
                seconds: 900,
                nanos: 0,
            }
        );
    }

    #[test]
    fn test_timestamp_minus_nano_overflow() {
        let timestamp = prost_types::Timestamp {
            seconds: 1,
            nanos: 500_000_000, // 500 milliseconds
        };
        let duration = prost_types::Duration {
            seconds: 0,
            nanos: 800_000_000, // 800 milliseconds
        };

        assert_eq!(
            timestamp_minus(&timestamp, &duration),
            prost_types::Timestamp {
                seconds: 0,
                nanos: 700_000_000, // 700 milliseconds
            }
        );
    }

    #[test]
    fn test_timestamp_minus_negative() {
        let timestamp = prost_types::Timestamp {
            seconds: 1,
            nanos: 500_000_000, // 500 milliseconds
        };
        let duration = prost_types::Duration {
            seconds: 1,
            nanos: 800_000_000, // 800 milliseconds
        };

        assert_eq!(
            timestamp_minus(&timestamp, &duration),
            prost_types::Timestamp {
                seconds: -1,
                nanos: 700_000_000, // 700 milliseconds
            }
        );
    }
}
