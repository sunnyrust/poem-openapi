use crate::{
    serde_json::Value,
    types::{DataType, ParseError, ParseResult, Type},
};

impl<T: Type> Type for Vec<T> {
    const DATA_TYPE: DataType = DataType::Array(&T::DATA_TYPE);

    fn parse(value: Option<Value>) -> ParseResult<Self> {
        match value.unwrap_or_default() {
            Value::Array(values) => {
                let mut res = Vec::with_capacity(values.len());
                for value in values {
                    res.push(T::parse(Some(value)).map_err(ParseError::propagate)?);
                }
                Ok(res)
            }
            value => Err(ParseError::expected_type(value)),
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
