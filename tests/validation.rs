use poem::{
    http::{StatusCode, Uri},
    Endpoint, IntoEndpoint, Request,
};
use poem_openapi::{registry::MetaApi, types::ParseFromJSON, Object, OpenApi, OpenApiService};
use serde_json::json;

#[test]
fn test_multiple_of() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(multiple_of = "10")]
        n: i32,
    }

    assert_eq!(A::parse_from_json(json!({ "n": 20 })).unwrap(), A { n: 20 });
    assert_eq!(
        A::parse_from_json(json!({ "n": 25 }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. multipleOf(10)"
    );
}

#[test]
fn test_maximum() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(maximum(value = "500"))]
        n: i32,
    }

    assert_eq!(
        A::parse_from_json(json!({ "n": 400 })).unwrap(),
        A { n: 400 }
    );
    assert_eq!(
        A::parse_from_json(json!({ "n": 500 })).unwrap(),
        A { n: 500 }
    );
    assert_eq!(
        A::parse_from_json(json!({ "n": 530 }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. maximum(500, exclusive: false)"
    );
}

#[test]
fn test_maximum_exclusive() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(maximum(value = "500", exclusive))]
        n: i32,
    }

    assert_eq!(
        A::parse_from_json(json!({ "n": 400 })).unwrap(),
        A { n: 400 }
    );
    assert_eq!(
        A::parse_from_json(json!({ "n": 500 }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. maximum(500, exclusive: true)"
    );
    assert_eq!(
        A::parse_from_json(json!({ "n": 530 }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `n` verification failed. maximum(500, exclusive: true)"
    );
}

#[test]
fn test_max_length() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(max_length = "5")]
        value: String,
    }

    assert_eq!(
        A::parse_from_json(json!({ "value": "abcd" })).unwrap(),
        A {
            value: "abcd".to_string()
        }
    );
    assert_eq!(
        A::parse_from_json(json!({ "value": "abcdef" }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `value` verification failed. maxLength(5)"
    );
}

#[test]
fn test_min_length() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(min_length = "5")]
        value: String,
    }

    assert_eq!(
        A::parse_from_json(json!({ "value": "abcdef" })).unwrap(),
        A {
            value: "abcdef".to_string()
        }
    );
    assert_eq!(
        A::parse_from_json(json!({ "value": "abcd" }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `value` verification failed. minLength(5)"
    );
}

#[test]
fn test_pattern() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(pattern = r#"\[.*\]"#)]
        value: String,
    }

    assert_eq!(
        A::parse_from_json(json!({ "value": "[123]" })).unwrap(),
        A {
            value: "[123]".to_string()
        }
    );
    assert_eq!(
        A::parse_from_json(json!({ "value": "123" }))
            .unwrap_err()
            .into_message(),
        r#"failed to parse "A": field `value` verification failed. pattern("\[.*\]")"#
    );
}

#[test]
fn test_max_items() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(max_items = "3")]
        values: Vec<String>,
    }

    assert_eq!(
        A::parse_from_json(json!({ "values": ["1", "2", "3"] })).unwrap(),
        A {
            values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        }
    );
    assert_eq!(
        A::parse_from_json(json!({ "values": ["1", "2", "3", "4"] }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `values` verification failed. maxItems(3)"
    );
}

#[test]
fn test_min_items() {
    #[derive(Object, Debug, Eq, PartialEq)]
    struct A {
        #[oai(min_items = "4")]
        values: Vec<String>,
    }

    assert_eq!(
        A::parse_from_json(json!({ "values": ["1", "2", "3", "4"] })).unwrap(),
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
        A::parse_from_json(json!({ "values": ["1", "2", "3"] }))
            .unwrap_err()
            .into_message(),
        "failed to parse \"A\": field `values` verification failed. minItems(4)"
    );
}

#[tokio::test]
async fn param_validator() {
    struct Api;

    #[OpenApi]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(
            &self,
            #[oai(name = "v", in = "query", maximum(value = "100", exclusive))] _v: i32,
        ) {
        }
    }

    let api = OpenApiService::new(Api).into_endpoint();
    let mut resp = api
        .call(Request::builder().uri(Uri::from_static("/?v=999")).finish())
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.take_body().into_string().await.unwrap(),
        "failed to parse param `v`: verification failed. maximum(100, exclusive: true)"
    );

    let meta: MetaApi = Api::meta().remove(0);
    assert_eq!(
        meta.paths[0].operations[0].params[0]
            .schema
            .unwrap_inline()
            .maximum,
        Some(100.0)
    );
    assert_eq!(
        meta.paths[0].operations[0].params[0]
            .schema
            .unwrap_inline()
            .exclusive_maximum,
        Some(true)
    );

    let resp = api
        .call(Request::builder().uri(Uri::from_static("/?v=50")).finish())
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}
