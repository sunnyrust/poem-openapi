use crate::{
    registry::{MetaSchema, MetaSchemaRef},
    serde_json::Value,
    types::{ParseError, ParseResult, Type, TypeName},
};

impl<T: Type> Type for Vec<T> {
    const NAME: TypeName = TypeName::Array(&T::NAME);

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(MetaSchema {
            items: Some(Box::new(T::schema_ref())),
            ..MetaSchema::new("array")
        })
    }

    fn parse(value: Value) -> ParseResult<Self> {
        match value {
            Value::Array(values) => {
                let mut res = Vec::with_capacity(values.len());
                for value in values {
                    res.push(T::parse(value).map_err(ParseError::propagate)?);
                }
                Ok(res)
            }
            _ => Err(ParseError::expected_type(value)),
        }
    }

    fn parse_from_str(_value: Option<&str>) -> ParseResult<Self> {
        Err(ParseError::not_support_parsing_from_string())
    }

    fn to_value(&self) -> Value {
        let mut values = Vec::with_capacity(self.len());
        for item in self {
            values.push(item.to_value());
        }
        Value::Array(values)
    }
}
