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

use crate::{
    registry::Registry,
    serde::{ser::SerializeMap, Serializer},
};

/// Represents a OpenAPI data type.
///
/// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#dataTypes>
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DataType {
    /// Normal data type.
    Normal {
        /// Represents a OpenAPI data type.
        ty: &'static str,

        /// Represents a OpenAPI data format.
        format: Option<&'static str>,
    },
    /// Enum data type.
    Enum {
        /// Represents all items of an enumerated type.
        items: &'static [&'static str],
    },
    /// Array data type.
    Array(&'static DataType),
    /// Schema reference.
    SchemaReference(&'static str),
}

impl Default for DataType {
    fn default() -> Self {
        Self::STRING
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Normal { ty, format } => match format {
                Some(format) => write!(f, "{}(${})", ty, format),
                None => write!(f, "{}", ty),
            },
            DataType::Enum { .. } => f.write_str("string"),
            DataType::Array(data_type) => write!(f, "[{}]", data_type),
            DataType::SchemaReference(schema) => write!(f, "{}", schema),
        }
    }
}

impl Serialize for DataType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_map(None)?;

        match self {
            DataType::Normal { ty, format } => {
                s.serialize_entry("type", ty)?;
                if let Some(format) = format {
                    s.serialize_entry("format", format)?;
                }
            }
            DataType::Array(data_type) => {
                s.serialize_entry("type", "array")?;
                s.serialize_entry("items", data_type)?;
            }
            DataType::Enum { items } => {
                s.serialize_entry("type", "string")?;
                s.serialize_entry("enum", items)?;
            }
            DataType::SchemaReference(schema_ref) => {
                s.serialize_entry("$ref", &format!("#/components/schemas/{}", schema_ref))?;
            }
        }

        s.end()
    }
}

impl DataType {
    /// A string type.
    pub const STRING: DataType = DataType::Normal {
        ty: "string",
        format: None,
    };

    /// A binary type.
    pub const BINARY: DataType = DataType::Normal {
        ty: "binary",
        format: None,
    };

    /// A object type.
    pub const OBJECT: DataType = DataType::Normal {
        ty: "object",
        format: None,
    };
}

/// Represents a OpenAPI type.
pub trait Type: Sized + Send + Sync {
    /// Data type of this value.
    const DATA_TYPE: DataType;

    /// If it is `true`, it means that this value is required.
    const IS_REQUIRED: bool = true;

    /// Parse from [`serde_json::Value`].
    fn parse(value: Value) -> ParseResult<Self>;

    /// Parse from string.
    fn parse_from_str(value: Option<&str>) -> ParseResult<Self>;

    /// Convert this value to [`serde_json::Value`].
    fn to_value(&self) -> Value;

    /// Register this type to types registry.
    #[allow(unused_variables)]
    fn register(registry: &mut Registry) {}
}
