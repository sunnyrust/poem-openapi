use serde_json::Value;

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseResult, Type, TypeName},
};

/// A password type.
///
/// NOTE: Its type is `string` and the format is `password`, and it does not
/// protect the data in the memory.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Password(pub String);

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Type for Password {
    const NAME: TypeName = TypeName::Normal {
        ty: "string",
        format: Some("password"),
    };

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Self::NAME.into())
    }

    fn parse(value: Value) -> ParseResult<Self> {
        if let Value::String(value) = value {
            Ok(Self(value))
        } else {
            Err(ParseError::expected_type(value))
        }
    }

    fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => Ok(Self(value.to_string())),
            None => Err(ParseError::expected_input()),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.clone())
    }
}
