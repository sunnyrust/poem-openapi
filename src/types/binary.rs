use poem::Error;
use serde_json::Value;

use super::{DataType, ParseError, ParseResult, Type};

/// Represents a binary data encoded with base64.
pub struct Base64(pub Vec<u8>);

impl Type for Base64 {
    const DATA_TYPE: DataType = DataType::new("string").with_format("byte");

    fn parse(value: Option<Value>) -> ParseResult<Self> {
        if let Some(Value::String(value)) = value {
            Ok(Self(base64::decode(value).map_err(Error::bad_request)?))
        } else {
            Err(ParseError::expected_type(value.unwrap_or_default()))
        }
    }

    fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => Ok(Self(base64::decode(value)?.into())),
            None => Err(ParseError::expected_input()),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(base64::encode(&self.0))
    }
}
