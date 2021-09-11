use crate::{
    registry::{MetaSchema, MetaSchemaRef},
    serde_json::Value,
    types::{ParseError, ParseFromJSON, ParseResult, ToJSON, Type, TypeName},
};

impl<T: Type> Type for Vec<T> {
    const NAME: TypeName = TypeName::Array(&T::NAME);

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(MetaSchema {
            items: Some(Box::new(T::schema_ref())),
            ..MetaSchema::new("array")
        })
    }

    impl_value_type!();
}

impl<T: ParseFromJSON> ParseFromJSON for Vec<T> {
    fn parse_from_json(value: Value) -> ParseResult<Self> {
        match value {
            Value::Array(values) => {
                let mut res = Vec::with_capacity(values.len());
                for value in values {
                    res.push(T::parse_from_json(value).map_err(ParseError::propagate)?);
                }
                Ok(res)
            }
            _ => Err(ParseError::expected_type(value)),
        }
    }
}

impl<T: ToJSON> ToJSON for Vec<T> {
    fn to_json(&self) -> Value {
        let mut values = Vec::with_capacity(self.len());
        for item in self {
            values.push(item.to_json());
        }
        Value::Array(values)
    }
}
