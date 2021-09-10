use poem::{Error, IntoResponse, Response, Result};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{
    payload::Payload,
    registry::{MetaSchemaRef, Registry},
    types::{ParseFromJSON, ToJSON},
    ParseRequestError,
};

/// A JSON payload.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Json<T>(pub T);

#[poem::async_trait]
impl<T: ParseFromJSON> Payload for Json<T> {
    const CONTENT_TYPE: &'static str = "application/json";

    fn schema_ref() -> MetaSchemaRef {
        T::schema_ref()
    }

    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {
        T::register(registry)
    }

    async fn parse(mut reader: impl AsyncRead + Send + Unpin + 'static) -> Result<Self> {
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .await
            .map_err(Error::bad_request)?;
        let value = serde_json::from_slice::<Value>(&data).map_err(|err| {
            Error::bad_request(ParseRequestError::ParseRequestBody {
                type_name: T::NAME,
                reason: err.to_string(),
            })
        })?;
        let value = T::parse_from_json(value).map_err(|err| {
            Error::bad_request(ParseRequestError::ParseRequestBody {
                type_name: T::NAME,
                reason: err.into_message(),
            })
        })?;
        Ok(Self(value))
    }
}

impl<T: ToJSON> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        poem::web::Json(self.0.to_json()).into_response()
    }
}
