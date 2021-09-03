use poem_openapi::{
    payload::{Json, PlainText},
    registry::{MetaMediaType, MetaRequest, MetaSchemaRef},
    types::DataType,
    Request, Schema,
};

#[derive(Debug, Schema, Eq, PartialEq)]
struct CreateUser {
    user: String,
    password: String,
}

/// MyRequest
///
/// ABC
#[derive(Debug, Request, Eq, PartialEq)]
enum MyRequest {
    CreateByJson(Json<CreateUser>),
    CreateByPlainText(PlainText),
}

#[tokio::test]
async fn test_meta() {
    assert_eq!(
        MyRequest::META,
        &MetaRequest {
            description: Some("MyRequest\n\nABC"),
            content: &[
                MetaMediaType {
                    content_type: "application/json",
                    schema: MetaSchemaRef::Reference("CreateUser"),
                },
                MetaMediaType {
                    content_type: "text/plain",
                    schema: MetaSchemaRef::Inline(DataType::STRING),
                }
            ],
            required: true
        }
    );
}

#[tokio::test]
async fn test_request() {
    let request = poem::Request::builder()
        .content_type("application/json")
        .body(
            serde_json::to_vec(&serde_json::json!({
                "user": "sunli",
                "password": "123456",
            }))
            .unwrap(),
        );
    let (request, mut body) = request.split();
    assert_eq!(
        MyRequest::from_request(&request, &mut body).await.unwrap(),
        MyRequest::CreateByJson(Json(CreateUser {
            user: "sunli".to_string(),
            password: "123456".to_string()
        }))
    );

    let request = poem::Request::builder()
        .content_type("text/plain")
        .body("abcdef".to_string());
    let (request, mut body) = request.split();
    assert_eq!(
        MyRequest::from_request(&request, &mut body).await.unwrap(),
        MyRequest::CreateByPlainText(PlainText("abcdef".to_string()))
    );
}
