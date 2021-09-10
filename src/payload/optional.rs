use poem::{Request, RequestBody};

use crate::{payload::Payload, registry::MetaSchemaRef, ParseRequestError};

#[poem::async_trait]
impl<T: Payload> Payload for Option<T> {
    const CONTENT_TYPE: &'static str = T::CONTENT_TYPE;
    const IS_REQUIRED: bool = false;

    fn schema_ref() -> MetaSchemaRef {
        T::schema_ref()
    }

    async fn from_request(
        request: &Request,
        body: &mut RequestBody,
    ) -> Result<Self, ParseRequestError> {
        if body.is_some() {
            Ok(Some(T::from_request(request, body).await?))
        } else {
            Ok(None)
        }
    }
}
