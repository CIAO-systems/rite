use model::BoxedError;

const CFG_FILTER_START_DATE: &str = "filter.start_date";
const CFG_FILTER_END_DATE: &str = "filter.end_date";
const CFG_FILTER_UPDATED_FROM: &str = "filter.updated_from";
const CFG_FILTER_UPDATED_TO: &str = "filter.updated_to";
const CFG_FILTER_EMPLOYEES: &str = "filter.employees";

#[derive(Debug)]
pub struct AbsencesFilter {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub updated_from: Option<String>,
    pub updated_to: Option<String>,
    pub employees: Option<Vec<i32>>,
}

impl AbsencesFilter {
    pub fn new() -> Self {
        Self {
            start_date: None,
            end_date: None,
            updated_from: None,
            updated_to: None,
            employees: None,
        }
    }

    pub fn load(config: &model::xml::config::Configuration) -> Result<Self, BoxedError> {
        Ok(Self {
            start_date: config.get(CFG_FILTER_START_DATE),
            end_date: config.get(CFG_FILTER_END_DATE),
            updated_from: config.get(CFG_FILTER_UPDATED_FROM),
            updated_to: config.get(CFG_FILTER_UPDATED_TO),
            employees: config.get_list(CFG_FILTER_EMPLOYEES),
        })
    }
}

#[cfg(test)]
mod tests;
