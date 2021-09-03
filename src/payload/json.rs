use poem::{Error, IntoResponse, Request, Response, Result};
use serde_json::Value;

use crate::{
    payload::Payload,
    poem::{FromRequest, RequestBody},
    registry::Registry,
    types::{DataType, Type},
    ParseRequestError,
};

/// A JSON payload.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Json<T>(pub T);

#[poem::async_trait]
impl<T: Type> Payload for Json<T> {
    const CONTENT_TYPE: &'static str = "application/json";
    const DATA_TYPE: &'static DataType = &T::DATA_TYPE;

    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {
        T::register(registry)
    }

    async fn from_request(request: &Request, body: &mut RequestBody) -> Result<Self> {
        let value = poem::web::Json::<Value>::from_request(request, body)
            .await
            .map_err(Error::bad_request)?;
        let value = T::parse(Some(value.0)).map_err(|err| {
            Error::bad_request(ParseRequestError::ParseRequestBody {
                data_type: &T::DATA_TYPE,
                reason: err.into_message(),
            })
        })?;
        Ok(Self(value))
    }
}

impl<T: Type> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        poem::web::Json(self.0.to_value()).into_response()
    }
}
