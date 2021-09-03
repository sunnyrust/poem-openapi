use chrono::{DateTime, FixedOffset};
use serde_json::Value;

use crate::types::{DataType, ParseError, ParseResult, Type};

impl Type for DateTime<FixedOffset> {
    const DATA_TYPE: DataType = DataType::Normal {
        ty: "string",
        format: Some("data-time"),
    };

    fn parse(value: Option<Value>) -> ParseResult<Self> {
        if let Some(Value::String(value)) = value {
            Ok(value.parse()?)
        } else {
            Err(ParseError::expected_type(value.unwrap_or_default()))
        }
    }

    fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => Ok(value.parse()?),
            None => Err(ParseError::expected_input()),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_rfc3339())
    }
}
