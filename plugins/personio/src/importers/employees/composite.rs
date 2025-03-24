use model::{record::Record, value::Value};

use crate::importers::employees::macros;

pub fn get_holiday_calendar(
    employee: &Box<personio_rs::personnel::models::Employee>,
) -> Option<Record> {
    if let Some(holiday_calendar) = &employee.holiday_calendar {
        if let Some(value) = &holiday_calendar.value {
            if let Some(attributes) = &value.attributes {
                let mut record = Record::new();

                // id
                macros::add_field_direct!(record, attributes, id);

                // country
                macros::add_field_option!(record, attributes, country);

                // name
                macros::add_field_option!(record, attributes, name);

                // state
                macros::add_field_option!(record, attributes, state);

                return Some(record);
            }
        }
    }
    None
}

pub fn get_supervisor(employee: &Box<personio_rs::personnel::models::Employee>) -> Option<Record> {
    if let Some(supervisor) = &employee.supervisor {
        if let Some(value) = &supervisor.value {
            if let Some(attributes) = &value.attributes {
                let mut record = Record::new();

                // id
                macros::add_field_boxed!(record, attributes, id);

                // email
                macros::add_field_boxed!(record, attributes, email);

                // first_name
                macros::add_field_boxed!(record, attributes, first_name);

                // last_name
                macros::add_field_boxed!(record, attributes, last_name);

                return Some(record);
            }
        }
    }
    None
}

fn create_office_record(office: &Box<personio_rs::personnel::models::Office>) -> Option<Record> {
    if let Some(value) = &office.value {
        if let Some(attributes) = &value.attributes {
            let mut record = Record::new();
            // name
            macros::add_field_option!(record, attributes, name);

            return Some(record);
        }
    }
    None
}

pub fn get_subcompany(attr: &Box<personio_rs::personnel::models::Employee>) -> Option<Record> {
    if let Some(subcompany) = &attr.subcompany {
        return create_office_record(subcompany);
    }
    None
}

pub fn get_office(employee: &Box<personio_rs::personnel::models::Employee>) -> Option<Record> {
    if let Some(office) = &employee.office {
        return create_office_record(office);
    }
    None
}

pub fn get_department(employee: &Box<personio_rs::personnel::models::Employee>) -> Option<Record> {
    if let Some(department) = &employee.department {
        if let Some(value) = &department.value {
            if let Some(attributes) = &value.attributes {
                let mut record = Record::new();

                // id
                macros::add_field_none!(record, attributes, id);

                // name
                macros::add_field_none!(record, attributes, name);

                return Some(record);
            }
        }
    }
    None
}

pub fn get_cost_centers(
    employee: &Box<personio_rs::personnel::models::Employee>,
) -> Option<Vec<Value>> {
    if let Some(cost_centers) = &employee.cost_centers {
        if let Some(ref cost_centers) = cost_centers.value {
            let mut result = Vec::new();
            for cost_center in cost_centers {
                if let Some(ref attributes) = cost_center.attributes {
                    let mut record = Record::new();

                    // id
                    macros::add_field_direct!(record, attributes, id);

                    // percentage
                    macros::add_field_direct!(record, attributes, percentage);

                    // name
                    macros::add_field_option!(record, attributes, name);

                    result.push(Value::from(record));
                }
            }
            return Some(result);
        }
    }
    None
}

pub fn get_work_schedule(
    employee: &Box<personio_rs::personnel::models::Employee>,
) -> Option<Record> {
    if let Some(ref work_schedule) = employee.work_schedule {
        if let Some(ref value) = work_schedule.value {
            if let Some(ref attributes) = value.attributes {
                let mut record = Record::new();

                macros::add_field_direct!(record, attributes, id);
                macros::add_field_option!(record, attributes, name);
                macros::add_field_option!(record, attributes, monday);
                macros::add_field_option!(record, attributes, tuesday);
                macros::add_field_option!(record, attributes, wednesday);
                macros::add_field_option!(record, attributes, thursday);
                macros::add_field_option!(record, attributes, friday);
                macros::add_field_option!(record, attributes, saturday);
                macros::add_field_option!(record, attributes, sunday);

                return Some(record);
            }
        }
    }
    None
}

pub fn get_absence_entitlement(
    employee: &Box<personio_rs::personnel::models::Employee>,
) -> Option<Vec<Value>> {
    if let Some(ref absence_entitlements) = employee.absence_entitlement {
        let mut result = Vec::new();
        for absence_entitlement in &absence_entitlements.value {
            if let Some(ref attributes) = absence_entitlement.attributes {
                let mut record = Record::new();

                macros::add_field_direct!(record, attributes, id);
                macros::add_field_option!(record, attributes, entitlement);
                macros::add_field_option!(record, attributes, name);

                result.push(Value::from(record));
            }
        }
        return Some(result);
    }
    None
}

pub fn get_team(employee: &Box<personio_rs::personnel::models::Employee>) -> Option<Record> {
    if let Some(ref team) = employee.team {
        if let Some(ref team) = team.value {
            if let Some(ref attributes) = team.attributes {
                let mut record = Record::new();

                macros::add_field_option!(record, attributes, name);

                return Some(record);
            }
        }
    }
    None
}
