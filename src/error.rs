use thiserror::Error;

/// This type represents errors that occur when parsing the HTTP request.
#[derive(Debug, Error)]
pub enum ParseRequestError {
    /// Failed to parse a parameter.
    #[error("failed to parse param `{name}`: {reason}")]
    ParseParam {
        /// The name of the parameter.
        name: String,

        /// The reason for the error.
        reason: String,
    },

    /// Failed to parse a schema.
    #[error("failed to parse schema: {reason}")]
    ParseSchema {
        /// The name of the schema.
        schema: &'static str,

        /// The reason for the error.
        reason: String,
    },

    /// The `Content-Type` requested by the client is not supported.
    #[error("the content type `{content_type}` is not supported.")]
    ContentTypeNotSupported {
        /// The `Content-Type` header requested by the client.
        content_type: String,
    },

    /// The client request does not include the `Content-Type` header.
    #[error("expect a `Content-Type` header.")]
    ExpectContentType,
}
