use crate::{
    payload::Payload,
    poem::{FromRequest, Request, RequestBody},
    registry::{MetaSchema, MetaSchemaRef},
    ParseRequestError,
};

/// A payload that writes the contents to a temporary file.
pub struct TempFile(poem::web::TempFile);

impl TempFile {
    /// Convert itself to the inner `poem::web::TempFile`.
    pub fn into_inner(self) -> poem::web::TempFile {
        self.0
    }
}

#[poem::async_trait]
impl Payload for TempFile {
    const CONTENT_TYPE: &'static str = "application/octet-stream";

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(MetaSchema::new("binary"))
    }

    async fn from_request(
        request: &Request,
        body: &mut RequestBody,
    ) -> Result<Self, ParseRequestError> {
        if body.is_some() {
            poem::web::TempFile::from_request(request, body)
                .await
                .map(Self)
                .map_err(|err| ParseRequestError::ParseRequestBody {
                    reason: err.to_string(),
                })
        } else {
            Err(ParseRequestError::ParseRequestBody {
                reason: "expect request body".to_string(),
            })
        }
    }
}
