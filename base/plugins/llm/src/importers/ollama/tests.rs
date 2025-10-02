use std::fs;

use model::import::handlers::CollectingRecordHandler;
use model::BoxedError;
use rig::{completion::Prompt, providers::ollama};
use serde_json::Value;

use crate::common::{extract_json_structures, response::LLMResponse};

use super::handle_response;

const TEXT_WITH_JSON1: &str = r#"Here is a raw JSON array containing random German first and last names:

```json
[
  {
    "first": "Christoph",
    "last": "Peter"
  },
  {
    "first": "Maria",
    "last": "Anna"
  },
  {
    "first": "Lena",
    "last": "Haus"
  },
  {
    "first": "Hans",
    "last": "Müller"
  },
  {
    "first": "Julia",
    "last": "Braun"
  },
  {
    "first": "Alexander",
    "last": "Witt"
  },
  {
    "first": "Sophie",
    "last": "Keller"
  },
  {
    "first": "Felix",
    "last": "Hahn"
  },
  {
    "first": "Eva",
    "last": "Richter"
  },
  {
    "first": "Lukas",
    "last": "Zimmerman"
  }
]
```"#;
const TEXT_WITH_JSON2: &str =
    r#"Some text {"key": "value"} and some more text [1, 2, 3] and more text"#;

#[tokio::test]
#[ignore = "for manual testing, because it is too slow and it needs an Ollama running"]
async fn test_ollama() -> Result<(), Box<dyn std::error::Error>> {
    // To start a local Ollama, you can use the script
    // [ollama.sh](https://github.com/CIAO-systems/assets-scripts/blob/main/internal/ollama.sh)

    // Create a new Ollama client (defaults to http://localhost:11434)
    let client = ollama::Client::from_url("http://localhost:11434");

    // Create an agent
    let agent = client
        .agent("deepseek-r1:7b")
        .preamble("Always answer in form of a raw (no markdown) JSON list of records with key/value pairs. Do not add any notes!")
        .build();

    let response = agent
        .prompt("List ten European cities with the number of people living there.")
        .await?;
    let response = LLMResponse::new(&response)?;
    if let Some(thinking) = response.thinking {
        println!("LLM was thinking about this:\n{thinking}\n\n")
    }
    if let Some(response) = response.response {
        println!("LLM response was:\n{response}");
    }

    Ok(())
}

#[test]
fn test_extract_json_array() -> Result<(), BoxedError> {
    let values = extract_json_structures(TEXT_WITH_JSON1)?;
    assert_eq!(1, values.len());
    assert!(values[0].is_array());

    if let Some(array) = values[0].as_array() {
        println!("Extracted JSON:\n{:?}", array);
        assert_eq!(10, array.len());

        let first = array.first().unwrap();
        let first = first.as_object().unwrap();
        assert_eq!("Christoph", first["first"]);
        assert_eq!("Peter", first["last"]);

        let last = array.last().unwrap();
        let last = last.as_object().unwrap();
        assert_eq!("Lukas", last["first"]);
        assert_eq!("Zimmerman", last["last"]);
    } else {
        panic!("No JSON found in the input.");
    }

    Ok(())
}

#[test]
fn test_extract_json_structures() -> Result<(), BoxedError> {
    let objects = extract_json_structures(TEXT_WITH_JSON1)?;
    assert_eq!(1, objects.len());

    let objects = extract_json_structures(TEXT_WITH_JSON2)?;
    assert_eq!(2, objects.len());

    // Element 0 is a object
    let object = objects[0].as_object().unwrap();
    let keys: Vec<String> = object.keys().cloned().collect();
    assert_eq!("key", keys[0]);
    let values: Vec<Value> = object.values().cloned().collect();
    assert_eq!("value", values[0].as_str().unwrap());

    // Element 1 is an array
    let array = objects[1].as_array().unwrap();
    assert_eq!(3, array.len());
    for (i, v) in array.iter().enumerate() {
        assert_eq!(
            i as u64,
            v.as_u64().unwrap() - 1,
            "Index should be value - 1"
        );
    }

    Ok(())
}

static EXAMPLE_RESPONSE: &str = "../../data/test/llm/example-response.json";

#[test]
fn test_handle_response() -> Result<(), Box<dyn std::error::Error>> {
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);

    let response = fs::read_to_string(EXAMPLE_RESPONSE)?;

    handle_response(response, &mut handler)?;

    assert_eq!(10, records.len(), "There should be 10 records");

    assert_field(&records, 0, "first", "Anna");
    assert_field(&records, 0, "last", "Becker");

    assert_field(&records, 1, "first", "Lukas");
    assert_field(&records, 1, "last", "Müller");

    assert_field(&records, 2, "first", "Hannah");
    assert_field(&records, 2, "last", "Spinner");

    assert_field(&records, 3, "first", "Manuel");
    assert_field(&records, 3, "last", "Frick");

    assert_field(&records, 4, "first", "Sophia");
    assert_field(&records, 4, "last", "Grimm");

    assert_field(&records, 5, "first", "Carolin");
    assert_field(&records, 5, "last", "Wunderlich");

    assert_field(&records, 6, "first", "Florian");
    assert_field(&records, 6, "last", "Gstall");

    assert_field(&records, 7, "first", "Julia");
    assert_field(&records, 7, "last", "Huber");

    assert_field(&records, 8, "first", "Theresa");
    assert_field(&records, 8, "last", "Weber");

    assert_field(&records, 9, "first", "Dennis");
    assert_field(&records, 9, "last", "Pflaum");
    Ok(())
}

fn assert_field(
    records: &Vec<model::record::Record>,
    index: usize,
    field_name: &str,
    field_value: &str,
) {
    assert_eq!(
        records[index].field_by_name(field_name).unwrap().value(),
        model::value::Value::String(field_value.to_string())
    );
}
