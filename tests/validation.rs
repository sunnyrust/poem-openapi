use poem_openapi::{types::Type, Schema};
use serde_json::json;

#[test]
fn test_multiple_of() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(multiple_of = "10")]
        n: i32,
    }

    assert_eq!(A::parse(Some(json!({ "n": 20 }))).unwrap(), A { n: 20 });
    assert_eq!(
        A::parse(Some(json!({ "n": 25 })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. multipleOf(10)"
    );
}

#[test]
fn test_maximum() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(maximum(value = "500"))]
        n: i32,
    }

    assert_eq!(A::parse(Some(json!({ "n": 400 }))).unwrap(), A { n: 400 });
    assert_eq!(A::parse(Some(json!({ "n": 500 }))).unwrap(), A { n: 500 });
    assert_eq!(
        A::parse(Some(json!({ "n": 530 })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. maximum(500, exclusive: false)"
    );
}

#[test]
fn test_maximum_exclusive() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(maximum(value = "500", exclusive))]
        n: i32,
    }

    assert_eq!(A::parse(Some(json!({ "n": 400 }))).unwrap(), A { n: 400 });
    assert_eq!(
        A::parse(Some(json!({ "n": 500 })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. maximum(500, exclusive: true)"
    );
    assert_eq!(
        A::parse(Some(json!({ "n": 530 })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. maximum(500, exclusive: true)"
    );
}

#[test]
fn test_max_length() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(max_length = "5")]
        value: String,
    }

    assert_eq!(
        A::parse(Some(json!({ "value": "abcd" }))).unwrap(),
        A {
            value: "abcd".to_string()
        }
    );
    assert_eq!(
        A::parse(Some(json!({ "value": "abcdef" })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `value` verification failed. maxLength(5)"
    );
}

#[test]
fn test_min_length() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(min_length = "5")]
        value: String,
    }

    assert_eq!(
        A::parse(Some(json!({ "value": "abcdef" }))).unwrap(),
        A {
            value: "abcdef".to_string()
        }
    );
    assert_eq!(
        A::parse(Some(json!({ "value": "abcd" })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `value` verification failed. minLength(5)"
    );
}

#[test]
fn test_pattern() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(pattern = r#"\[.*\]"#)]
        value: String,
    }

    assert_eq!(
        A::parse(Some(json!({ "value": "[123]" }))).unwrap(),
        A {
            value: "[123]".to_string()
        }
    );
    assert_eq!(
        A::parse(Some(json!({ "value": "123" })))
            .unwrap_err()
            .into_message(),
        r#"failed to parse "A": field `value` verification failed. pattern("\[.*\]")"#
    );
}

#[test]
fn test_max_items() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(max_items = "3")]
        values: Vec<String>,
    }

    assert_eq!(
        A::parse(Some(json!({ "values": ["1", "2", "3"] }))).unwrap(),
        A {
            values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        }
    );
    assert_eq!(
        A::parse(Some(json!({ "values": ["1", "2", "3", "4"] })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `values` verification failed. maxItems(3)"
    );
}

#[test]
fn test_min_items() {
    #[derive(Schema, Debug, Eq, PartialEq)]
    struct A {
        #[oai(min_items = "4")]
        values: Vec<String>,
    }

    assert_eq!(
        A::parse(Some(json!({ "values": ["1", "2", "3", "4"] }))).unwrap(),
        A {
            values: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string()
            ],
        }
    );
    assert_eq!(
        A::parse(Some(json!({ "values": ["1", "2", "3"] })))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `values` verification failed. minItems(4)"
    );
}
