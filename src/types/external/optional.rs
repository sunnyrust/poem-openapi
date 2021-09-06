use serde_json::Value;

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseResult, Type, TypeName},
};

impl<T: Type> Type for Option<T> {
    const NAME: TypeName = T::NAME;
    const IS_REQUIRED: bool = false;

    fn schema_ref() -> MetaSchemaRef {
        T::schema_ref()
    }

    fn parse(value: Value) -> ParseResult<Self> {
        match value {
            Value::Null => Ok(None),
            value => Ok(Some(T::parse(value).map_err(ParseError::propagate)?)),
        }
    }

    fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => T::parse_from_str(Some(value))
                .map_err(ParseError::propagate)
                .map(Some),
            None => Ok(None),
        }
    }

    fn to_value(&self) -> Value {
        match self {
            Some(value) => value.to_value(),
            None => Value::Null,
        }
    }
}
