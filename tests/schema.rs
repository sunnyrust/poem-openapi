use indexmap::{IndexMap, IndexSet};
use poem_openapi::{
    registry::{MetaProperty, MetaSchema, Registry},
    types::Type,
    Schema,
};

/// simple schema
///
/// A
/// B
///
/// C
#[derive(Schema, Debug, Eq, PartialEq)]
struct Simple {
    /// field a
    a: i32,

    /// field b
    ///
    /// A
    /// B
    ///
    /// C
    b: String,

    c: Option<i32>,
}

#[test]
fn parse_simple() {
    let obj = Simple {
        a: 10,
        b: "abc".to_string(),
        c: None,
    };
    assert_eq!(
        obj.to_value(),
        serde_json::json!({
            "a": 10,
            "b": "abc",
            "c": null,
        })
    );

    assert_eq!(
        Simple::parse(Some(serde_json::json!({
            "a": 10,
            "b": "abc",
            "c": null,
        })))
        .unwrap(),
        obj
    );
}

#[test]
fn test_meta() {
    let mut registry = Registry::default();
    Simple::register(&mut registry);

    let meta_schema: &MetaSchema = registry.schemas.get("Simple").unwrap();
    assert_eq!(meta_schema.summary.as_deref(), Some("simple schema"));
    assert_eq!(meta_schema.description.as_deref(), Some("A\nB\n\nC"));
    assert_eq!(meta_schema.deprecated, false);
    assert_eq!(
        meta_schema.properties,
        std::iter::IntoIterator::into_iter([
            (
                "a",
                MetaProperty {
                    data_type: i32::DATA_TYPE,
                    description: Some("field a"),
                    default: None,
                    validators: Default::default()
                },
            ),
            (
                "b",
                MetaProperty {
                    data_type: String::DATA_TYPE,
                    description: Some("field b\n\nA\nB\n\nC"),
                    default: None,
                    validators: Default::default()
                },
            ),
            (
                "c",
                MetaProperty {
                    data_type: i32::DATA_TYPE,
                    description: None,
                    default: None,
                    validators: Default::default()
                },
            ),
        ])
        .collect::<IndexMap<_, _>>()
    );
    assert_eq!(
        meta_schema.required,
        std::iter::IntoIterator::into_iter(["a", "b"]).collect::<IndexSet<_>>()
    );
}
