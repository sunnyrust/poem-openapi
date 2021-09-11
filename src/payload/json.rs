use poem::{FromRequest, IntoResponse, Request, RequestBody, Response};
use serde_json::Value;

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

    async fn from_request(
        request: &Request,
        body: &mut RequestBody,
    ) -> Result<Self, ParseRequestError> {
        if body.is_some() {
            let value = poem::web::Json::<Value>::from_request(request, body)
                .await
                .map_err(|err| ParseRequestError::ParseRequestBody {
                    reason: err.to_string(),
                })?;
            let value =
                T::parse_from_json(value.0).map_err(|err| ParseRequestError::ParseRequestBody {
                    reason: err.into_message(),
                })?;
            Ok(Self(value))
        } else {
            Err(ParseRequestError::ParseRequestBody {
                reason: "expect request body".to_string(),
            })
        }
    }
}

impl<T: ToJSON> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        poem::web::Json(self.0.to_json()).into_response()
    }
}
