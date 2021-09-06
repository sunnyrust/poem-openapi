//! Commonly used types.

mod binary;
mod error;
mod external;
mod password;

use std::fmt::{self, Display, Formatter};

pub use binary::Base64;
pub use error::{ParseError, ParseResult};
pub use password::Password;
use serde_json::Value;

use crate::registry::{MetaSchemaRef, Registry};

/// Represents a type name.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TypeName {
    /// Normal type name.
    Normal {
        /// Type name
        ty: &'static str,

        /// Format name
        format: Option<&'static str>,
    },
    /// The type name of array.
    Array(&'static TypeName),
}

impl Display for TypeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeName::Normal { ty, format } => match format {
                Some(format) => write!(f, "{}(${})", ty, format),
                None => write!(f, "{}", ty),
            },
            TypeName::Array(ty) => {
                write!(f, "[{}]", ty)
            }
        }
    }
}

/// Represents a OpenAPI type.
pub trait Type: Sized + Send + Sync {
    /// The name of this type.
    const NAME: TypeName;

    /// If it is `true`, it means that this value is required.
    const IS_REQUIRED: bool = true;

    /// Get schema reference of this type.
    fn schema_ref() -> MetaSchemaRef;

    /// Register this type to types registry.
    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {}

    /// Parse from [`serde_json::Value`].
    fn parse(value: Value) -> ParseResult<Self>;

    /// Parse from string.
    fn parse_from_str(value: Option<&str>) -> ParseResult<Self>;

    /// Convert this value to [`serde_json::Value`].
    fn to_value(&self) -> Value;
}
