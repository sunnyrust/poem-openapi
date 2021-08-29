use poem::{Error, IntoResponse, Request, Response, Result};
use serde_json::Value;

use crate::{
    base::Schema,
    payload::Payload,
    poem::{FromRequest, RequestBody},
    registry::{MetaSchemaRef, Registry},
    ParseRequestError,
};

/// A JSON payload.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Json<T>(pub T);

#[poem::async_trait]
impl<T: Schema> Payload for Json<T> {
    const CONTENT_TYPE: &'static str = "application/json";
    const SCHEMA_REF: MetaSchemaRef = MetaSchemaRef::Reference(T::NAME);

    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {
        T::register(registry)
    }

    async fn from_request(request: &Request, body: &mut RequestBody) -> Result<Self> {
        let value = poem::web::Json::<Value>::from_request(request, body)
            .await
            .map_err(Error::bad_request)?;
        let value = T::parse(Some(value.0)).map_err(|err| {
            Error::bad_request(ParseRequestError::ParseSchema {
                schema: T::NAME,
                reason: err.into_message(),
            })
        })?;
        Ok(Self(value))
    }
}

impl<T: Schema> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        poem::web::Json(self.0.to_value()).into_response()
    }
}
