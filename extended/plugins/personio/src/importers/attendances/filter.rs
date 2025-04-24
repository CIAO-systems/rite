use model::BoxedError;

const CFG_FILTER_START_DATE: &str = "filter.start_date";
const CFG_FILTER_END_DATE: &str = "filter.end_date";
const CFG_FILTER_UPDATED_FROM: &str = "filter.updated_from";
const CFG_FILTER_UPDATED_TO: &str = "filter.updated_to";
const CFG_FILTER_INCLUDE_PENDING: &str = "filter.includePending";
const CFG_FILTER_EMPLOYEES: &str = "filter.employees";

pub struct AttendancesFilter {
    pub start_date: String,
    pub end_date: String,
    pub updated_from: Option<String>,
    pub updated_to: Option<String>,
    pub include_pending: Option<bool>,
    pub employees: Option<Vec<i32>>,
}

impl AttendancesFilter {
    pub fn new() -> Self {
        Self {
            start_date: "".to_string(),
            end_date: "".to_string(),
            updated_from: None,
            updated_to: None,
            include_pending: None,
            employees: None,
        }
    }

    pub fn load(config: &model::xml::config::Configuration) -> Result<Self, BoxedError> {
        let start_date = config.get_result(CFG_FILTER_START_DATE)?;
        let end_date = config.get_result(CFG_FILTER_END_DATE)?;

        Ok(Self {
            start_date,
            end_date,
            updated_from: config.get(CFG_FILTER_UPDATED_FROM),
            updated_to: config.get(CFG_FILTER_UPDATED_TO),
            include_pending: config.get_bool(CFG_FILTER_INCLUDE_PENDING),
            employees: config.get_list(CFG_FILTER_EMPLOYEES),
        })
    }
}

#[cfg(test)]
mod tests;
