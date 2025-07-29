use crate::test_utils::*;

#[test]
fn minimal_create_response_request() -> Result<(), serde_json::Error> {
    let example_value = r#"{
      "model": "gpt-4",
      "input": [
        {"role": "user", "content": "Hello!"}
      ]
    }"#;

    round_trip_test::<crate::CreateResponse>(example_value)?;

    let example_value_2 = r#"{
      "model": "gpt-4",
      "input": [
        {"role": "user", "content": "Hello!", "type": "message"}
      ]
    }"#;

    round_trip_test::<crate::CreateResponse>(example_value_2)
}

#[test]
fn minimal_create_response_request_with_str() -> Result<(), serde_json::Error> {
    let example_value = r#"{
      "model": "gpt-4",
      "input": "Hello!"
    }"#;

    round_trip_test::<crate::CreateResponse>(example_value)
}
