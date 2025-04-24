// FIXME DEVICES-24 we need to think about, if ths is needed!

// use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
// use crate::ciao::devices::Device;

// fn summer() -> NaiveDateTime {
//     NaiveDateTime::new(
//         NaiveDate::from_ymd_opt(2024, 06, 01).unwrap(),
//         NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
//     )
// }

// fn winter() -> NaiveDateTime {
//     NaiveDateTime::new(
//         NaiveDate::from_ymd_opt(2024, 12, 01).unwrap(),
//         NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
//     )
// }

// #[test]
// fn test_get_time_zone_offset() {
//     let now = &Utc::now().naive_utc();

//     let device = Device::with_time_zone("Etc/UTC");
//     assert_eq!(0, device.get_time_zone_offset(now).unwrap_or_default());

//     let device = Device::with_time_zone("Europe/Berlin");
//     assert_eq!(
//         7200,
//         device.get_time_zone_offset(&summer()).unwrap_or_default()
//     );

//     assert_eq!(
//         3600,
//         device.get_time_zone_offset(&winter()).unwrap_or_default()
//     );

//     let device = Device::with_time_zone("America/New_York");
//     assert_eq!(
//         -14400,
//         device.get_time_zone_offset(&summer()).unwrap_or_default()
//     );

//     assert_eq!(
//         -18000,
//         device.get_time_zone_offset(&winter()).unwrap_or_default()
//     );

//     let device = Device::with_time_zone("Europe/Lisbon");
//     assert_eq!(
//         3600,
//         device.get_time_zone_offset(&summer()).unwrap_or_default()
//     );

//     assert_eq!(
//         0,
//         device.get_time_zone_offset(&winter()).unwrap_or_default()
//     );

//     let device = Device::with_time_zone("Europe/Tallinn");
//     assert_eq!(
//         10800,
//         device.get_time_zone_offset(&summer()).unwrap_or_default()
//     );

//     assert_eq!(
//         7200,
//         device.get_time_zone_offset(&winter()).unwrap_or_default()
//     );
// }
