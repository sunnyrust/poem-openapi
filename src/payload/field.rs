use std::ops::Deref;

use poem::{Request, RequestBody};

use crate::{payload::Payload, registry::MetaSchemaRef, ParseRequestError};

/// A payload for multipart fields.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field<T> {
    payload: T,
    content_type: Option<String>,
    file_name: Option<String>,
}

impl<T> Field<T> {
    /// Convert itself to the inner `T`.
    #[inline]
    pub fn into_inner(self) -> T {
        self.payload
    }

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

impl<T> Deref for Field<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

#[poem::async_trait]
impl<T: Payload> Payload for Field<T> {
    const CONTENT_TYPE: &'static str = T::CONTENT_TYPE;
    const IS_REQUIRED: bool = T::IS_REQUIRED;

    fn schema_ref() -> MetaSchemaRef {
        T::schema_ref()
    }

    async fn from_request(
        request: &Request,
        body: &mut RequestBody,
    ) -> Result<Self, ParseRequestError> {
        let content_type = request.content_type().map(ToString::to_string);
        let file_name = request
            .headers()
            .get("poem-filename")
            .and_then(|value| value.to_str().ok())
            .map(ToString::to_string);
        T::from_request(request, body).await.map(|payload| Self {
            payload,
            content_type,
            file_name,
        })
    }
}
