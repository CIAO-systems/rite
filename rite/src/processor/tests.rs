use super::create_variables;
use crate::processor::RITE_CONFIG_PATH;

#[test]
fn test_create_variables_relative1() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir().unwrap_or("".into());
    let variables = create_variables("data/filename.xml");

    let value = variables.get(RITE_CONFIG_PATH);
    assert!(value.is_some());
    assert_eq!(String::from(format!("{}/data", cwd.display())), *value.unwrap(),);

    Ok(())
}

#[test]
fn test_create_variables_relative2() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir().unwrap_or("".into());
    let cwdp = cwd.parent().unwrap_or(&cwd);

    let variables = create_variables("../data/filename.xml");

    let value = variables.get(RITE_CONFIG_PATH);
    assert!(value.is_some());
    assert_eq!(String::from(format!("{}/data", cwdp.display())), *value.unwrap(),);

    Ok(())
}

#[test]
fn test_create_variables_relative3() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir().unwrap_or("".into());
    let variables = create_variables("./data/filename.xml");

    let value = variables.get(RITE_CONFIG_PATH);
    assert!(value.is_some());
    assert_eq!(String::from(format!("{}/data", cwd.display())), *value.unwrap(),);

    Ok(())
}

#[test]
fn test_create_variables_relative4() -> Result<(), Box<dyn std::error::Error>> {
    let variables = create_variables("/data/filename.xml");

    let value = variables.get(RITE_CONFIG_PATH);
    assert!(value.is_some());
    assert_eq!(String::from("/data"), *value.unwrap(),);

    Ok(())
}
