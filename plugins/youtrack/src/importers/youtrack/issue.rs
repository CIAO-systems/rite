use serde::{Deserialize, Serialize};

use super::common::{Project, User};

/// rust struct for youTrack Issue JSON
/// See https://www.jetbrains.com/help/youtrack/devportal/resource-api-issues.html
#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    #[serde(rename = "idReadable")]
    pub id_readable: Option<String>,
    #[serde(rename = "commentsCount")]
    pub comments_count: Option<i32>,
    pub description: Option<String>,
    pub created: Option<i64>,
    #[serde(rename = "draftOwner")]
    pub draft_owner: Option<User>,
    #[serde(rename = "isDraft")]
    pub is_draft: Option<bool>,
    #[serde(rename = "numberInProject")]
    pub number_in_project: Option<i32>,
    pub project: Option<Project>,
    /// The timestamp in milliseconds indicating the moment when the issue
    /// was assigned a state that is considered to be resolved
    pub resolved: Option<i64>,
    pub summary: Option<String>,
    /// The timestamp in milliseconds indicating the last update of the issue
    pub updated: Option<i64>,
    pub updater: Option<User>,
    pub votes: Option<i16>,
    #[serde(rename = "wikifiedDescription")]
    pub wikified_description: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::importers::youtrack::factory::YouTrackObject;

    static TEST_DATA: &str = r#"
[
  {
    "description": null,
    "summary": "Sprint 3. Task 2",
    "reporter": {
      "login": "root",
      "$type": "User"
    },
    "idReadable": "SP-38",
    "id": "2-42",
    "$type": "Issue"
  },
  {
    "description": "Let's create new issue from REST API",
    "summary": "Issue from REST #1",
    "reporter": {
      "login": "root",
      "$type": "User"
    },
    "idReadable": "SP-7",
    "id": "2-6",
    "$type": "Issue"
  }
]    
    "#;

    #[test]
    fn test_automatic_type_creation() -> Result<(), Box<dyn std::error::Error>> {
        let json: Value = serde_json::from_str(TEST_DATA)?;
        // println!("{:#?}", json);

        assert!(json.is_array());

        match json.as_array() {
            Some(array) => {
                // Get object type from first element of the array
                assert!(array.len() > 0);
                // Iterate over the array
                for element in array {
                    // Create a rust object from the JSON, based on $type
                    let x = YouTrackObject::from_type(element)?;
                    println!("{:#?}", x);
                }
            }
            None => panic!("Not an array"),
        }

        Ok(())
    }
}
