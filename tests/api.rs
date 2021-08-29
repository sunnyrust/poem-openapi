use std::collections::HashMap;

use poem::{
    http::{Method, StatusCode, Uri},
    Endpoint, Error, IntoEndpoint,
};
use poem_openapi::{
    payload::{Json, PlainText},
    registry::{
        MetaAPI, MetaMediaType, MetaOperation, MetaOperationParam, MetaParamIn, MetaPath,
        MetaRequest, MetaResponse, MetaResponses, MetaSchemaRef,
    },
    types::DataType,
    OpenAPI, Request, Response, Schema, API,
};
use tokio::sync::Mutex;

/// Create user schema
#[derive(Debug, Schema, Eq, PartialEq)]
struct CreateUser {
    user: String,
    password: String,
}

/// Create a new user request
#[derive(Debug, Request, Eq, PartialEq)]
enum CreateUserRequest {
    CreateByJson(Json<CreateUser>),
    CreateByPlainText(PlainText),
}

#[derive(Response)]
#[oai(bad_request_handler = "bad_request_handler")]
enum CreateUserResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok,
    /// Returns when the user already exists.
    #[oai(status = 409)]
    UserAlreadyExists,
    /// Returns when the request parameters is incorrect.
    #[oai(status = 400)]
    BadRequest(PlainText),
}

fn bad_request_handler(err: Error) -> CreateUserResponse {
    CreateUserResponse::BadRequest(PlainText(format!("error: {}", err)))
}

#[derive(Default)]
struct Api {
    users: Mutex<HashMap<String, String>>,
}

/// Test API
///
/// A
/// B
#[API]
impl Api {
    /// Create a new user
    ///
    /// A
    /// B
    ///
    /// C
    #[oai(path = "/users", method = "post")]
    #[allow(unused_variables)]
    async fn create_user(
        &self,
        #[oai(name = "key", in = "query", desc = "api key")] key: String,
        #[oai(name = "X-API-TOKEN", in = "header", deprecated)] api_token: Option<String>,
        req: CreateUserRequest,
    ) -> CreateUserResponse {
        let mut users = self.users.lock().await;

        match req {
            CreateUserRequest::CreateByJson(req) => {
                if users.contains_key(&req.0.user) {
                    return CreateUserResponse::UserAlreadyExists;
                }
                users.insert(req.0.user, req.0.password);
                CreateUserResponse::Ok
            }
            CreateUserRequest::CreateByPlainText(req) => {
                let s = req.0.split(':').collect::<Vec<_>>();
                if s.len() != 2 {
                    return CreateUserResponse::BadRequest("invalid plain text request".into());
                }

                if users.contains_key(s[0]) {
                    return CreateUserResponse::UserAlreadyExists;
                }
                users.insert(s[0].to_string(), s[1].to_string());
                CreateUserResponse::Ok
            }
        }
    }

    #[oai(path = "/test_payload_request", method = "post")]
    async fn test_payload_request(&self, payload: PlainText) -> PlainText {
        payload
    }

    #[oai(path = "/test_payload_response", method = "get")]
    async fn test_payload_response(&self) -> PlainText {
        PlainText("abc".to_string())
    }

    #[oai(path = "/test_unit_response", method = "get")]
    async fn test_unit_response(&self) {}

    #[oai(path = "/test_path_param/:userId", method = "get")]
    async fn test_path_param(
        &self,
        #[oai(name = "userId", in = "path")] user_id: String,
    ) -> PlainText {
        PlainText(user_id.into())
    }

    #[oai(path = "/test_header_param", method = "get")]
    async fn test_header(
        &self,
        #[oai(name = "X-TOKEN", in = "header")] token: String,
    ) -> PlainText {
        token.into()
    }

    #[oai(path = "/test_opt_header_param", method = "get")]
    async fn test_opt_header(
        &self,
        #[oai(name = "X-TOKEN", in = "header")] token: Option<String>,
    ) -> PlainText {
        token.unwrap_or_else(|| "def".to_string()).into()
    }
}

#[test]
fn test_api_meta() {
    assert_eq!(
        Api::metadata(),
        vec![MetaAPI {
            paths: &[
                MetaPath {
                    path: "/users",
                    operations: &[MetaOperation {
                        method: Method::POST,
                        tags: &[],
                        summary: Some("Create a new user"),
                        description: Some("A\nB\n\nC"),
                        params: &[
                            MetaOperationParam {
                                name: "key",
                                schema: DataType::STRING,
                                in_type: MetaParamIn::Query,
                                description: Some("api key"),
                                required: true,
                                deprecated: false,
                            },
                            MetaOperationParam {
                                name: "X-API-TOKEN",
                                schema: DataType::STRING,
                                in_type: MetaParamIn::Header,
                                description: None,
                                required: false,
                                deprecated: true,
                            }
                        ],
                        request: Some(&MetaRequest {
                            description: Some("Create a new user request"),
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
                        }),
                        responses: &MetaResponses {
                            responses: &[
                                MetaResponse {
                                    description: Some(
                                        "Returns when the user is successfully created."
                                    ),
                                    status: Some(200),
                                    content: &[]
                                },
                                MetaResponse {
                                    description: Some("Returns when the user already exists."),
                                    status: Some(409),
                                    content: &[]
                                },
                                MetaResponse {
                                    description: Some(
                                        "Returns when the request parameters is incorrect."
                                    ),
                                    status: Some(400),
                                    content: &[MetaMediaType {
                                        content_type: "text/plain",
                                        schema: MetaSchemaRef::Inline(DataType::STRING),
                                    }]
                                }
                            ]
                        },
                        deprecated: false
                    }]
                },
                MetaPath {
                    path: "/test_payload_request",
                    operations: &[MetaOperation {
                        method: Method::POST,
                        tags: &[],
                        summary: None,
                        description: None,
                        params: &[],
                        request: Some(&MetaRequest {
                            description: None,
                            content: &[MetaMediaType {
                                content_type: "text/plain",
                                schema: MetaSchemaRef::Inline(DataType::STRING),
                            }],
                            required: true
                        }),
                        responses: &MetaResponses {
                            responses: &[MetaResponse {
                                description: None,
                                status: Some(200),
                                content: &[MetaMediaType {
                                    content_type: "text/plain",
                                    schema: MetaSchemaRef::Inline(DataType::STRING),
                                }]
                            }]
                        },
                        deprecated: false
                    }]
                },
                MetaPath {
                    path: "/test_payload_response",
                    operations: &[MetaOperation {
                        method: Method::GET,
                        tags: &[],
                        summary: None,
                        description: None,
                        params: &[],
                        request: None,
                        responses: &MetaResponses {
                            responses: &[MetaResponse {
                                description: None,
                                status: Some(200),
                                content: &[MetaMediaType {
                                    content_type: "text/plain",
                                    schema: MetaSchemaRef::Inline(DataType::STRING),
                                }]
                            }]
                        },
                        deprecated: false
                    }]
                },
                MetaPath {
                    path: "/test_unit_response",
                    operations: &[MetaOperation {
                        method: Method::GET,
                        tags: &[],
                        summary: None,
                        description: None,
                        params: &[],
                        request: None,
                        responses: &MetaResponses {
                            responses: &[MetaResponse {
                                description: None,
                                status: Some(200),
                                content: &[]
                            }]
                        },
                        deprecated: false
                    }]
                },
                MetaPath {
                    path: "/test_path_param/{userId}",
                    operations: &[MetaOperation {
                        method: Method::GET,
                        tags: &[],
                        summary: None,
                        description: None,
                        params: &[MetaOperationParam {
                            name: "userId",
                            schema: DataType::STRING,
                            in_type: MetaParamIn::Path,
                            description: None,
                            required: true,
                            deprecated: false
                        }],
                        request: None,
                        responses: &MetaResponses {
                            responses: &[MetaResponse {
                                description: None,
                                status: Some(200),
                                content: &[MetaMediaType {
                                    content_type: "text/plain",
                                    schema: MetaSchemaRef::Inline(DataType::STRING),
                                }]
                            }]
                        },
                        deprecated: false
                    }]
                },
                MetaPath {
                    path: "/test_header_param",
                    operations: &[MetaOperation {
                        method: Method::GET,
                        tags: &[],
                        summary: None,
                        description: None,
                        params: &[MetaOperationParam {
                            name: "X-TOKEN",
                            schema: DataType::STRING,
                            in_type: MetaParamIn::Header,
                            description: None,
                            required: true,
                            deprecated: false
                        }],
                        request: None,
                        responses: &MetaResponses {
                            responses: &[MetaResponse {
                                description: None,
                                status: Some(200),
                                content: &[MetaMediaType {
                                    content_type: "text/plain",
                                    schema: MetaSchemaRef::Inline(DataType::STRING),
                                }]
                            }]
                        },
                        deprecated: false
                    }]
                },
                MetaPath {
                    path: "/test_opt_header_param",
                    operations: &[MetaOperation {
                        method: Method::GET,
                        tags: &[],
                        summary: None,
                        description: None,
                        params: &[MetaOperationParam {
                            name: "X-TOKEN",
                            schema: DataType::STRING,
                            in_type: MetaParamIn::Header,
                            description: None,
                            required: false,
                            deprecated: false
                        }],
                        request: None,
                        responses: &MetaResponses {
                            responses: &[MetaResponse {
                                description: None,
                                status: Some(200),
                                content: &[MetaMediaType {
                                    content_type: "text/plain",
                                    schema: MetaSchemaRef::Inline(DataType::STRING),
                                }]
                            }]
                        },
                        deprecated: false
                    }]
                }
            ],
        }]
    );
}

#[tokio::test]
async fn test_call() {
    let api = OpenAPI::new(Api::default()).into_endpoint();

    let resp = api
        .call(
            poem::Request::builder()
                .method(Method::POST)
                .uri(Uri::from_static("/users?key=abc123"))
                .content_type("application/json")
                .body(
                    serde_json::to_vec(&CreateUser {
                        user: "sunli".to_string(),
                        password: "123456".to_string(),
                    })
                    .unwrap(),
                ),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = api
        .call(
            poem::Request::builder()
                .method(Method::POST)
                .uri(Uri::from_static("/users?key=abc123"))
                .content_type("application/json")
                .body(
                    serde_json::to_vec(&CreateUser {
                        user: "sunli".to_string(),
                        password: "123456".to_string(),
                    })
                    .unwrap(),
                ),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::POST)
                .uri(Uri::from_static("/users"))
                .content_type("application/json")
                .body(
                    serde_json::to_vec(&CreateUser {
                        user: "sunli".to_string(),
                        password: "123456".to_string(),
                    })
                    .unwrap(),
                ),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.take_body().into_string().await.unwrap(),
        r#"error: 400: failed to parse param `key`: Type "string" expects an input value."#
    );

    let resp = api
        .call(
            poem::Request::builder()
                .method(Method::POST)
                .uri(Uri::from_static("/users?key=abc123"))
                .content_type("text/plain")
                .body("abc:678"),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::POST)
                .uri(Uri::from_static("/test_payload_request"))
                .body("abcdef"),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.take_body().into_string().await.unwrap(), "abcdef");

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::GET)
                .uri(Uri::from_static("/test_payload_response"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.take_body().into_string().await.unwrap(), "abc");

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::GET)
                .uri(Uri::from_static("/test_unit_response"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.take_body().into_string().await.unwrap(), "");

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::GET)
                .uri(Uri::from_static("/test_path_param/abc"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.take_body().into_string().await.unwrap(), "abc");

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::GET)
                .header("X-TOKEN", "abc")
                .uri(Uri::from_static("/test_header_param"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.take_body().into_string().await.unwrap(), "abc");

    let resp = api
        .call(
            poem::Request::builder()
                .method(Method::GET)
                .uri(Uri::from_static("/test_header_param"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::GET)
                .header("X-TOKEN", "abc")
                .uri(Uri::from_static("/test_opt_header_param"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.take_body().into_string().await.unwrap(), "abc");

    let mut resp = api
        .call(
            poem::Request::builder()
                .method(Method::GET)
                .uri(Uri::from_static("/test_opt_header_param"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.take_body().into_string().await.unwrap(), "def");
}
