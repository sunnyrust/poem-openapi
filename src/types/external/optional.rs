use serde_json::Value;

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseFromJSON, ParseFromParameter, ParseResult, ToJSON, Type, TypeName},
};

impl<T: Type> Type for Option<T> {
    const NAME: TypeName = T::NAME;
    const IS_REQUIRED: bool = false;

    fn schema_ref() -> MetaSchemaRef {
        T::schema_ref()
    }
}

impl<T: ParseFromJSON> ParseFromJSON for Option<T> {
    fn parse_from_json(value: Value) -> ParseResult<Self> {
        match value {
            Value::Null => Ok(None),
            value => Ok(Some(
                T::parse_from_json(value).map_err(ParseError::propagate)?,
            )),
        }
    }
}

impl<T: ParseFromParameter> ParseFromParameter for Option<T> {
    fn parse_from_parameter(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => T::parse_from_parameter(Some(value))
                .map_err(ParseError::propagate)
                .map(Some),
            None => Ok(None),
        }
    }
}

impl<T: ToJSON> ToJSON for Option<T> {
    fn to_json(&self) -> Value {
        match self {
            Some(value) => value.to_json(),
            None => Value::Null,
        }
    }
}
