use chrono::{DateTime, FixedOffset};
use serde_json::Value;

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseResult, Type, TypeName},
};

impl Type for DateTime<FixedOffset> {
    const NAME: TypeName = TypeName::Normal {
        ty: "string",
        format: Some("data-time"),
    };

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Self::NAME.into())
    }

    fn parse(value: Value) -> ParseResult<Self> {
        if let Value::String(value) = value {
            Ok(value.parse()?)
        } else {
            Err(ParseError::expected_type(value))
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
