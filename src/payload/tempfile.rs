use std::{
    pin::Pin,
    task::{Context, Poll},
};

use poem::Result;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncSeekExt, ReadBuf, SeekFrom},
};

use crate::{
    payload::Payload,
    poem::Error,
    registry::{MetaSchema, MetaSchemaRef},
};

/// A payload that writes the contents to a temporary file.
#[cfg_attr(docsrs, doc(cfg(feature = "tempfile")))]
pub struct TempFile(File);

impl AsyncRead for TempFile {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

#[poem::async_trait]
impl Payload for TempFile {
    const CONTENT_TYPE: &'static str = "application/octet-stream";

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(MetaSchema::new("binary"))
    }

    async fn parse(mut reader: impl AsyncRead + Send + Unpin + 'static) -> Result<Self> {
        let mut file = tokio::fs::File::from_std(
            ::tempfile::tempfile().map_err(Error::internal_server_error)?,
        );
        tokio::io::copy(&mut reader, &mut file)
            .await
            .map_err(Error::bad_request)?;
        file.seek(SeekFrom::Start(0))
            .await
            .map_err(Error::bad_request)?;
        Ok(Self(file))
    }
}
