use poem::Error;
use serde_json::Value;

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseResult, Type, TypeName},
};

/// Represents a binary data encoded with base64.
pub struct Base64(pub Vec<u8>);

impl Type for Base64 {
    const NAME: TypeName = TypeName::Normal {
        ty: "string",
        format: Some("bytes"),
    };

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Self::NAME.into())
    }

    fn parse(value: Value) -> ParseResult<Self> {
        if let Value::String(value) = value {
            Ok(Self(base64::decode(value).map_err(Error::bad_request)?))
        } else {
            Err(ParseError::expected_type(value))
        }
    }

    fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => Ok(Self(base64::decode(value)?)),
            None => Err(ParseError::expected_input()),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(base64::encode(&self.0))
    }
}
