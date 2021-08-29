use serde_json::Value;

use super::{DataType, ParseError, ParseResult, Type};

/// A password type.
///
/// NOTE: Its type is `string` and the format is `password`, and it does not
/// protect the data in the memory.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Password(pub String);

impl Type for Password {
    const DATA_TYPE: DataType = DataType::new("string").with_format("password");

    fn parse(value: Option<Value>) -> ParseResult<Self> {
        if let Some(Value::String(value)) = value {
            Ok(Self(value))
        } else {
            Err(ParseError::expected_type(value.unwrap_or_default()))
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
