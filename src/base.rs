use poem::{route::Route, Error, IntoResponse, RequestBody, Result};

use crate::{
    payload::Payload,
    registry::{MetaAPI, MetaMediaType, MetaRequest, MetaResponse, MetaResponses, Registry},
    types::Type,
};

/// Represents a OpenAPI schema.
pub trait Schema: Type {
    /// The name of this schema type.
    const NAME: &'static str;
}

/// Represents a OpenAPI request object.
///
/// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#requestBodyObject>
#[poem::async_trait]
pub trait Request: Sized {
    /// The metadata of this request type.
    const META: &'static MetaRequest;

    /// Register the schema contained in this request object to the registry.
    fn register(registry: &mut Registry);

    /// Parse the request object from the HTTP request.
    async fn from_request(request: &poem::Request, body: &mut RequestBody) -> Result<Self>;
}

#[poem::async_trait]
impl<T: Payload> Request for T {
    const META: &'static MetaRequest = &MetaRequest {
        description: None,
        content: &[MetaMediaType {
            content_type: T::CONTENT_TYPE,
            schema: T::SCHEMA_REF,
        }],
        required: true,
    };

    fn register(registry: &mut Registry) {
        T::register(registry);
    }

    async fn from_request(request: &poem::Request, body: &mut RequestBody) -> Result<Self> {
        T::from_request(request, body).await
    }
}

/// Represents a OpenAPI responses object.
///
/// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#responsesObject>
pub trait Response: IntoResponse + Sized {
    /// The metadata of this request type.
    const META: &'static MetaResponses;

    /// If true, it means that the response object has a custom bad request
    /// handler.
    const BAD_REQUEST_HANDLER: bool;

    /// Register the schema contained in this response object to the registry.
    fn register(registry: &mut Registry);

    /// Convert [`Error`](poem::Error) to this response object.
    #[allow(unused_variables)]
    fn from_parse_request_error(err: Error) -> Self {
        unreachable!()
    }
}

impl Response for () {
    const META: &'static MetaResponses = &MetaResponses {
        responses: &[MetaResponse {
            description: None,
            status: Some(200),
            content: &[],
        }],
    };
    const BAD_REQUEST_HANDLER: bool = false;

    fn register(_registry: &mut Registry) {}
}

impl<T: Payload> Response for T {
    const META: &'static MetaResponses = &MetaResponses {
        responses: &[MetaResponse {
            description: None,
            status: Some(200),
            content: &[MetaMediaType {
                content_type: T::CONTENT_TYPE,
                schema: T::SCHEMA_REF,
            }],
        }],
    };
    const BAD_REQUEST_HANDLER: bool = false;

    fn register(registry: &mut Registry) {
        T::register(registry);
    }
}

/// Represents a OpenAPI object.
pub trait API: Sized {
    /// Gets metadata of this API object.
    fn metadata() -> Vec<MetaAPI>;

    /// Register some types to the registry.
    fn register(registry: &mut Registry);

    /// Adds all API endpoints to the routing object.
    fn add_routes(self, route: Route) -> Route;

    /// Combine two API objects into one.
    fn combine<T: API>(self, other: T) -> CombinedAPI<Self, T> {
        CombinedAPI(self, other)
    }
}

/// API for the [`combine`](API::combine) method.
pub struct CombinedAPI<A, B>(A, B);

impl<A: API, B: API> API for CombinedAPI<A, B> {
    fn metadata() -> Vec<MetaAPI> {
        let mut metadata = A::metadata();
        metadata.extend(B::metadata());
        metadata
    }

    fn register(registry: &mut Registry) {
        A::register(registry);
        B::register(registry);
    }

    fn add_routes(self, route: Route) -> Route {
        self.1.add_routes(self.0.add_routes(route))
    }
}
