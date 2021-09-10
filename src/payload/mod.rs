//! Commonly used payload types.

mod binary;
mod field;
mod json;
mod optional;
mod plain_text;
mod tempfile;

pub use binary::Binary;
pub use field::Field;
pub use json::Json;
pub use plain_text::PlainText;
use poem::{Request, RequestBody};
pub use tempfile::TempFile;

use crate::{
    registry::{MetaSchemaRef, Registry},
    ParseRequestError,
};

/// Represents a payload type.
#[poem::async_trait]
pub trait Payload: Sized {
    /// The content type of this payload.
    const CONTENT_TYPE: &'static str;

    /// If it is `true`, it means that this payload is required.
    const IS_REQUIRED: bool = true;

    /// Gets schema reference of this payload.
    fn schema_ref() -> MetaSchemaRef;

    /// Register the schema contained in this payload to the registry.
    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {}

    /// Parse the payload object from the HTTP request.
    async fn from_request(
        request: &Request,
        body: &mut RequestBody,
    ) -> Result<Self, ParseRequestError>;
}
