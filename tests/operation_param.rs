use poem::{
    http::{header, Method, StatusCode, Uri},
    web::Cookie,
    Endpoint, IntoEndpoint, Request,
};
use poem_openapi::{
    registry::{MetaAPI, MetaParamIn, MetaSchema, MetaSchemaRef},
    types::Type,
    OpenAPI, API,
};
use serde_json::json;

fn default_i32() -> i32 {
    999
}

#[tokio::test]
async fn param_name() {
    struct API;

    #[API]
    impl API {
        #[oai(path = "/abc", method = "get")]
        async fn test(&self, #[oai(name = "a", in = "query")] a: i32) {
            assert_eq!(a, 10);
        }
    }

    let meta: MetaAPI = API::meta().remove(0);
    assert_eq!(meta.paths[0].operations[0].params[0].name, "a");

    let ep = OpenAPI::new(API).into_endpoint();
    let resp = ep
        .call(
            Request::builder()
                .method(Method::GET)
                .uri(Uri::from_static("/abc?a=10"))
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn query() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "query")] v: i32) {
            assert_eq!(v, 10);
        }
    }

    let meta: MetaAPI = Api::meta().remove(0);
    assert_eq!(
        meta.paths[0].operations[0].params[0].in_type,
        MetaParamIn::Query
    );
    assert_eq!(meta.paths[0].operations[0].params[0].name, "v");

    let api = OpenAPI::new(Api).into_endpoint();
    let mut resp = api
        .call(Request::builder().uri(Uri::from_static("/?v=10")).finish())
        .await;
    println!("{}", resp.take_body().into_string().await.unwrap());
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn query_default() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "query", default = "default_i32")] v: i32) {
            assert_eq!(v, 999);
        }
    }

    let meta: MetaAPI = Api::meta().remove(0);
    assert_eq!(
        meta.paths[0].operations[0].params[0].in_type,
        MetaParamIn::Query
    );
    assert_eq!(meta.paths[0].operations[0].params[0].name, "v");
    assert_eq!(
        meta.paths[0].operations[0].params[0].schema,
        MetaSchemaRef::Inline(MetaSchema {
            format: Some("int32"),
            default: Some(json!(999)),
            ..i32::schema_ref().unwrap_inline().clone()
        })
    );

    let api = OpenAPI::new(Api).into_endpoint();
    let resp = api.call(Request::default()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn header() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "header")] v: i32) {
            assert_eq!(v, 10);
        }
    }

    let api = OpenAPI::new(Api).into_endpoint();
    let resp = api.call(Request::builder().header("v", 10).finish()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn header_default() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "header", default = "default_i32")] v: i32) {
            assert_eq!(v, 999);
        }
    }

    let api = OpenAPI::new(Api).into_endpoint();
    let resp = api.call(Request::default()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn path() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/k/:v", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "path")] v: i32) {
            assert_eq!(v, 10);
        }
    }

    let api = OpenAPI::new(Api).into_endpoint();
    let resp = api
        .call(Request::builder().uri(Uri::from_static("/k/10")).finish())
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn cookie() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "cookie")] v: i32) {
            assert_eq!(v, 10);
        }
    }

    let api = OpenAPI::new(Api).into_endpoint();
    let cookie = Cookie::new("v", "10");
    let resp = api
        .call(
            Request::builder()
                .header(header::COOKIE, cookie.to_string())
                .finish(),
        )
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn cookie_default() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "cookie", default = "default_i32")] v: i32) {
            assert_eq!(v, 999);
        }
    }

    let api = OpenAPI::new(Api).into_endpoint();
    let resp = api.call(Request::builder().finish()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn deprecated() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/a", method = "get")]
        async fn a(&self, #[oai(name = "v", in = "query", deprecated)] _v: i32) {
            todo!()
        }

        #[oai(path = "/b", method = "get")]
        async fn b(&self, #[oai(name = "v", in = "query")] _v: i32) {
            todo!()
        }
    }

    let meta: MetaAPI = Api::meta().remove(0);

    assert_eq!(meta.paths[0].path, "/a");
    assert!(meta.paths[0].operations[0].params[0].deprecated);

    assert_eq!(meta.paths[1].path, "/b");
    assert!(!meta.paths[1].operations[0].params[0].deprecated);
}

#[tokio::test]
async fn desc() {
    struct Api;

    #[API]
    impl Api {
        #[oai(path = "/", method = "get")]
        async fn test(&self, #[oai(name = "v", in = "query", desc = "ABC")] _v: i32) {
            todo!()
        }
    }

    let meta: MetaAPI = Api::meta().remove(0);
    assert_eq!(
        meta.paths[0].operations[0].params[0].in_type,
        MetaParamIn::Query
    );
    assert_eq!(meta.paths[0].operations[0].params[0].name, "v");
    assert_eq!(
        meta.paths[0].operations[0].params[0].description,
        Some("ABC")
    );
}
