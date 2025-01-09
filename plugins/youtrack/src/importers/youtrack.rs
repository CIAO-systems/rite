pub mod common;
pub mod issue;
pub mod work_item;

#[allow(dead_code)]
pub mod factory {

    #[derive(Debug)]
    pub enum YouTrackObject {
        Issue(super::issue::Issue),
        IssueWorkItem(super::work_item::IssueWorkItem),
        User(super::common::User),
        DurationValue(super::common::DurationValue),
        Project(super::common::Project),
        None,
    }

    impl YouTrackObject {
        pub fn from_type(element: &serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
            if let Some(object) = element.as_object() {
                let object_type = &object["$type"];

                if let Some(object_type) = object_type.as_str() {
                    return match object_type {
                        "Issue" => Ok(YouTrackObject::Issue(create_object(element)?)),
                        "IssueWorkItem" => {
                            Ok(YouTrackObject::IssueWorkItem(create_object(element)?))
                        }
                        _ => Ok(YouTrackObject::None),
                    };
                }
            }
            Ok(YouTrackObject::None)
        }
    }

    fn create_object<T: serde::de::DeserializeOwned>(
        element: &serde_json::Value,
    ) -> Result<T, Box<dyn std::error::Error>> {
        match serde_json::from_value::<T>(element.clone()) {
            Ok(object) => Ok(object),
            Err(e) => Err(e.into()),
        }
    }
}
