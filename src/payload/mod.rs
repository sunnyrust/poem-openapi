//! Commonly used payload types.

mod binary;
mod json;
mod plain_text;
#[cfg(feature = "tempfile")]
mod tempfile;

pub use binary::Binary;
pub use json::Json;
pub use plain_text::PlainText;
use poem::Result;
use tokio::io::AsyncRead;

#[cfg(feature = "tempfile")]
pub use self::tempfile::TempFile;
use crate::registry::{MetaSchemaRef, Registry};

/// Represents a payload type.
#[poem::async_trait]
pub trait Payload: Sized {
    /// The content type of this payload.
    const CONTENT_TYPE: &'static str;

    /// Gets schema reference of this payload.
    fn schema_ref() -> MetaSchemaRef;

    /// Register the schema contained in this payload to the registry.
    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {}

    /// Parse the payload from the reader.
    async fn parse(reader: impl AsyncRead + Send + Unpin + 'static) -> Result<Self>;
}
