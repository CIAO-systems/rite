use super::*;
use model::xml::config::Configuration;

#[test]
fn test_attendances_filter_load_success() {
    let mut config = Configuration::new();

    config.insert_str(CFG_FILTER_START_DATE, "2024-01-01");
    config.insert_str(CFG_FILTER_END_DATE, "2024-01-31");
    config.insert_str(CFG_FILTER_UPDATED_FROM, "2024-01-15");
    config.insert_str(CFG_FILTER_UPDATED_TO, "2024-01-20");
    config.insert_str(CFG_FILTER_INCLUDE_PENDING, "true");
    config.insert_str(CFG_FILTER_EMPLOYEES, "1, 2, 3");

    let filter = AttendancesFilter::load(&config).expect("Failed to load filter");
    assert_eq!(filter.start_date, "2024-01-01");
    assert_eq!(filter.end_date, "2024-01-31");
    assert_eq!(filter.updated_from, Some("2024-01-15".to_string()));
    assert_eq!(filter.updated_to, Some("2024-01-20".to_string()));
    assert_eq!(filter.include_pending, Some(true));
    assert_eq!(filter.employees, Some(vec![1, 2, 3]));
}

#[test]
fn test_attendances_filter_load_missing_optional_values() {
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_START_DATE, "2024-01-01");
    config.insert_str(CFG_FILTER_END_DATE, "2024-01-31");

    let filter = AttendancesFilter::load(&config).expect("Failed to load filter");
    assert_eq!(filter.start_date, "2024-01-01");
    assert_eq!(filter.end_date, "2024-01-31");
    assert_eq!(filter.updated_from, None);
    assert_eq!(filter.updated_to, None);
    assert_eq!(filter.include_pending, None);
    assert_eq!(filter.employees, None);
}

#[test]
fn test_attendances_filter_invalid_boolean() {
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_START_DATE, "2024-01-01");
    config.insert_str(CFG_FILTER_END_DATE, "2024-01-31");
    config.insert_str(CFG_FILTER_INCLUDE_PENDING, "invalid");

    let filter = AttendancesFilter::load(&config).expect("Failed to load filter");
    assert_eq!(filter.include_pending, None);
}

#[test]
fn test_attendances_filter_invalid_employee_ids() {
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_START_DATE, "2024-01-01");
    config.insert_str(CFG_FILTER_END_DATE, "2024-01-31");
    config.insert_str(CFG_FILTER_EMPLOYEES, "a, b, c");

    let filter = AttendancesFilter::load(&config).expect("Failed to load filter");
    assert_eq!(filter.employees, None);
}
