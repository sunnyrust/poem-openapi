//! Commonly used payload types.

mod json;
mod plain_text;

pub use json::Json;
pub use plain_text::PlainText;
use poem::{IntoResponse, Request, RequestBody, Result};

use crate::registry::{MetaSchemaRef, Registry};

/// Represents a payload type.
#[poem::async_trait]
pub trait Payload: IntoResponse + Sized {
    /// The content type of this payload.
    const CONTENT_TYPE: &'static str;

    /// The schema ref of this payload.
    const SCHEMA_REF: MetaSchemaRef;

    /// Register the schema contained in this payload to the registry.
    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {}

    /// Parse the payload object from the HTTP request.
    async fn from_request(request: &Request, body: &mut RequestBody) -> Result<Self>;
}
