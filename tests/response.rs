mod request;

use poem::{http::StatusCode, IntoResponse};
use poem_openapi::{
    payload::{Json, PlainText},
    registry::{MetaMediaType, MetaResponse, MetaResponses, MetaSchemaRef},
    types::DataType,
    Response, Schema,
};
use serde_json::Value;

#[derive(Schema)]
struct BadRequestResult {
    error_code: i32,
    message: String,
}

#[derive(Response)]
enum MyResponse {
    /// Ok
    #[oai(status = 200)]
    Ok,
    /// A
    /// B
    ///
    /// C
    #[oai(status = 400)]
    BadRequest(Json<BadRequestResult>),
    Default(StatusCode, PlainText),
}

#[tokio::test]
async fn test_response() {
    assert_eq!(
        MyResponse::META,
        &MetaResponses {
            responses: &[
                MetaResponse {
                    description: Some("Ok"),
                    status: Some(200),
                    content: &[],
                },
                MetaResponse {
                    description: Some("A\nB\n\nC"),
                    status: Some(400),
                    content: &[MetaMediaType {
                        content_type: "application/json",
                        schema: MetaSchemaRef::Reference("BadRequestResult")
                    }]
                },
                MetaResponse {
                    description: None,
                    status: None,
                    content: &[MetaMediaType {
                        content_type: "text/plain",
                        schema: MetaSchemaRef::Inline(DataType::STRING),
                    }]
                }
            ],
        },
    );

    let resp = MyResponse::Ok.into_response();
    assert_eq!(resp.status(), StatusCode::OK);

    let mut resp = MyResponse::BadRequest(Json(BadRequestResult {
        error_code: 123,
        message: "abc".to_string(),
    }))
    .into_response();
    assert_eq!(
        serde_json::from_slice::<Value>(&resp.take_body().into_bytes().await.unwrap()).unwrap(),
        serde_json::json!({
            "errorCode": 123,
            "message": "abc",
        })
    );
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let mut resp = MyResponse::Default(StatusCode::BAD_GATEWAY, PlainText("abcdef".to_string()))
        .into_response();
    assert_eq!(resp.take_body().into_string().await.unwrap(), "abcdef");
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
}
