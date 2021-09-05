use serde_json::Value;

use crate::types::{DataType, ParseError, ParseResult, Type};

impl Type for String {
    const DATA_TYPE: DataType = DataType::STRING;

    fn parse(value: Value) -> ParseResult<Self> {
        if let Value::String(value) = value {
            Ok(value)
        } else {
            Err(ParseError::expected_type(value))
        }
    }

    fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => Ok(value.to_string()),
            None => Err(ParseError::expected_input()),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.clone())
    }
}
