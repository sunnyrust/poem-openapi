use std::{
    fmt::{self, Debug, Formatter},
    pin::Pin,
    task::{Context, Poll},
};

use poem::web::Field as PoemField;
use tokio::{
    fs::File,
    io::{AsyncRead, ReadBuf},
};

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseFromMultipartField, ParseResult, Type, TypeName},
};

/// A uploaded file for multipart.
pub struct Upload {
    file_name: Option<String>,
    content_type: Option<String>,
    file: File,
}

impl Debug for Upload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Upload");
        if let Some(file_name) = self.file_name() {
            d.field("filename", &file_name);
        }
        if let Some(content_type) = self.content_type() {
            d.field("content_type", &content_type);
        }
        d.finish()
    }
}

impl Upload {
    /// Get the content type of the field.
    #[inline]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// The file name found in the `Content-Disposition` header.
    #[inline]
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }
}

impl AsyncRead for Upload {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.file).poll_read(cx, buf)
    }
}

impl Type for Upload {
    const NAME: TypeName = TypeName::Normal {
        ty: "string",
        format: Some("binary"),
    };

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Self::NAME.into())
    }

    impl_value_type!();
}

#[poem::async_trait]
impl ParseFromMultipartField for Upload {
    async fn parse_from_multipart(field: Option<PoemField>) -> ParseResult<Self> {
        match field {
            Some(field) => {
                let content_type = field.content_type().map(ToString::to_string);
                let file_name = field.file_name().map(ToString::to_string);
                Ok(Self {
                    content_type,
                    file_name,
                    file: field.tempfile().await.map_err(ParseError::custom)?,
                })
            }
            None => Err(ParseError::expected_input()),
        }
    }
}
