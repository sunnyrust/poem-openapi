use serde_json::Value;

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseResult, Type, TypeName},
};

impl Type for String {
    const NAME: TypeName = TypeName::Normal {
        ty: "string",
        format: None,
    };

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Self::NAME.into())
    }

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
