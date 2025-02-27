use std::collections::HashMap;

use model::xml::config::Configuration;

use crate::{
    com::atoss::atc::protobuf::filter::ParameterMetaData,
    importers::clock_records::{ATC_FILTER_EMPLOYEE, ATC_FILTER_TIMESTAMP},
};

use super::{add_employee_filter, add_period_filter, CFG_FILTER_EMPLOYEE, CFG_FILTER_PERIOD};

#[test]
fn test_add_employee_filter() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter_meta_data: HashMap<String, ParameterMetaData> = HashMap::new();
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_EMPLOYEE, "employee #1");
    add_employee_filter(&mut parameter_meta_data, &config)?;

    assert!(parameter_meta_data.contains_key(ATC_FILTER_EMPLOYEE));

    Ok(())
}

#[test]
fn test_add_period_filter_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter_meta_data: HashMap<String, ParameterMetaData> = HashMap::new();
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_PERIOD, "2024-01-01:2024-12-31");
    add_period_filter(&mut parameter_meta_data, &config)?;

    assert!(parameter_meta_data.contains_key(ATC_FILTER_TIMESTAMP));

    let filter = parameter_meta_data.get(ATC_FILTER_TIMESTAMP);
    assert!(filter.is_some());

    let lower = &filter.unwrap().first;
    assert!(lower.is_some());

    let upper = &filter.unwrap().upper;
    assert!(upper.is_some());

    Ok(())
}

#[test]
fn test_add_period_filter_start() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter_meta_data: HashMap<String, ParameterMetaData> = HashMap::new();
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_PERIOD, "2024-01-01:");
    add_period_filter(&mut parameter_meta_data, &config)?;

    assert!(parameter_meta_data.contains_key(ATC_FILTER_TIMESTAMP));

    let filter = parameter_meta_data.get(ATC_FILTER_TIMESTAMP);
    assert!(filter.is_some());

    let lower = &filter.unwrap().first;
    assert!(lower.is_some());

    let upper = &filter.unwrap().upper;
    assert!(upper.is_none());

    Ok(())
}

#[test]
fn test_add_period_filter_end() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter_meta_data: HashMap<String, ParameterMetaData> = HashMap::new();
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_PERIOD, ":2024-12-31");
    add_period_filter(&mut parameter_meta_data, &config)?;

    assert!(parameter_meta_data.contains_key(ATC_FILTER_TIMESTAMP));

    let filter = parameter_meta_data.get(ATC_FILTER_TIMESTAMP);
    assert!(filter.is_some());

    let lower = &filter.unwrap().first;
    assert!(lower.is_none());

    let upper = &filter.unwrap().upper;
    assert!(upper.is_some());

    Ok(())
}

#[test]
fn test_add_period_filter_none() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter_meta_data: HashMap<String, ParameterMetaData> = HashMap::new();
    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_PERIOD, ":");
    add_period_filter(&mut parameter_meta_data, &config)?;

    assert!(parameter_meta_data.contains_key(ATC_FILTER_TIMESTAMP));

    let filter = parameter_meta_data.get(ATC_FILTER_TIMESTAMP);
    assert!(filter.is_some());
    let lower = &filter.unwrap().first;
    assert!(lower.is_none());

    let upper = &filter.unwrap().upper;
    assert!(upper.is_none());
    Ok(())
}
