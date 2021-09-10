use poem::{IntoResponse, Response, Result};
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{
    payload::Payload, poem::Error, registry::MetaSchemaRef, types::Type, ParseRequestError,
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

    fn schema_ref() -> MetaSchemaRef {
        String::schema_ref()
    }

    async fn parse(mut reader: impl AsyncRead + Send + Unpin + 'static) -> Result<Self> {
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .await
            .map_err(Error::bad_request)?;
        let value = String::from_utf8(data).map_err(|err| {
            Error::bad_request(ParseRequestError::ParseRequestBody {
                type_name: String::NAME,
                reason: err.to_string(),
            })
        })?;
        Ok(Self(value))
    }
}

impl IntoResponse for PlainText {
    fn into_response(self) -> Response {
        Response::builder()
            .content_type(Self::CONTENT_TYPE)
            .body(self.0)
    }
}
