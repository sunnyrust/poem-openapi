use poem::{IntoResponse, Response, Result};
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::{
    payload::Payload,
    poem::Error,
    registry::{MetaSchema, MetaSchemaRef},
};

/// A binary payload.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Binary(pub Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for Binary {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[poem::async_trait]
impl Payload for Binary {
    const CONTENT_TYPE: &'static str = "application/octet-stream";

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(MetaSchema::new("binary"))
    }

    async fn parse(mut reader: impl AsyncRead + Send + Unpin + 'static) -> Result<Self> {
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .await
            .map_err(Error::bad_request)?;
        Ok(Self(data))
    }
}

impl IntoResponse for Binary {
    fn into_response(self) -> Response {
        Response::builder()
            .content_type(Self::CONTENT_TYPE)
            .body(self.0)
    }
}
