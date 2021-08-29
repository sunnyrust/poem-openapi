use poem::{IntoResponse, Request, Response, Result};

use crate::{
    payload::Payload,
    poem::{FromRequest, RequestBody},
    registry::MetaSchemaRef,
    types::DataType,
};

/// A UTF8 string payload.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlainText(pub String);

impl<T: Into<String>> From<T> for PlainText {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[poem::async_trait]
impl Payload for PlainText {
    const CONTENT_TYPE: &'static str = "text/plain";
    const SCHEMA_REF: MetaSchemaRef = MetaSchemaRef::Inline(DataType::STRING);

    async fn from_request(request: &Request, body: &mut RequestBody) -> Result<Self> {
        Ok(Self(String::from_request(request, body).await?))
    }
}

impl IntoResponse for PlainText {
    fn into_response(self) -> Response {
        Response::builder().content_type("text/plain").body(self.0)
    }
}
