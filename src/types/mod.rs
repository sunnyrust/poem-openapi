//! Commonly used types.

mod binary;
mod error;
mod external;
mod password;

use std::fmt::{self, Display, Formatter};

pub use binary::Base64;
pub use error::{ParseError, ParseResult};
pub use password::Password;
use serde::Serialize;
use serde_json::Value;

use crate::registry::Registry;

/// Represents a OpenAPI data type.
///
/// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#dataTypes>
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct DataType {
    /// Represents a OpenAPI data type.
    #[serde(rename = "type")]
    pub ty: &'static str,

    /// Represents a OpenAPI data format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<&'static str>,

    /// Represents all items of an enumerated type.
    #[serde(rename = "enum", skip_serializing_if = "<[_]>::is_empty")]
    pub enum_items: &'static [&'static str],
}

impl Default for DataType {
    fn default() -> Self {
        Self::STRING
    }
}

impl DataType {
    /// A string type.
    pub const STRING: DataType = DataType::new("string");

    /// A binary type.
    pub const BINARY: DataType = DataType::new("binary");

    /// Create a new data type.
    #[must_use]
    pub const fn new(ty: &'static str) -> DataType {
        Self {
            ty,
            format: None,
            enum_items: &[],
        }
    }

    /// Sets the format of this data type.
    #[must_use]
    pub const fn with_format(self, format: &'static str) -> Self {
        Self {
            format: Some(format),
            ..self
        }
    }

    /// Sets all items of enumeration type.
    #[must_use]
    pub const fn with_enum_items(self, items: &'static [&'static str]) -> Self {
        Self {
            enum_items: items,
            ..self
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.format {
            Some(format) => write!(f, "{}({})", self.ty, format),
            None => write!(f, "{}", self.ty),
        }
    }
}

/// Represents a OpenAPI type.
pub trait Type: Sized + Send + Sync {
    /// Data type of this value.
    const DATA_TYPE: DataType;

    /// If it is `true`, it means that this value is required.
    const IS_REQUIRED: bool = true;

    /// Parse from [`serde_json::Value`].
    fn parse(value: Option<Value>) -> ParseResult<Self>;

    /// Parse from string.
    fn parse_from_str(value: Option<&str>) -> ParseResult<Self>;

    /// Convert this value to [`serde_json::Value`].
    fn to_value(&self) -> Value;

    /// Register this type to types registry.
    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {}
}
