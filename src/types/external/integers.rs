use serde_json::Value;

use crate::types::{DataType, ParseError, ParseResult, Type};

macro_rules! impl_type_for_integers {
    ($(($ty:ty, $format:literal)),*) => {
        $(
        impl Type for $ty {
            const DATA_TYPE: DataType = DataType::new("integer");

            fn parse(value: Option<Value>) -> ParseResult<Self> {
                if let Some(Value::Number(n)) = value {
                    let n = n
                        .as_i64()
                        .ok_or_else(|| ParseError::from("invalid integer"))?;

                    if n < Self::MIN as i64 || n > Self::MAX as i64 {
                        return Err(ParseError::from(format!(
                            "Only integers from {} to {} are accepted.",
                            Self::MIN,
                            Self::MAX
                        )));
                    }

                    Ok(n as Self)
                } else {
                    Err(ParseError::expected_type(value.unwrap_or_default()))
                }
            }

            fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
                match value {
                    Some(value) => value.parse().map_err(|err| ParseError::custom(err)),
                    None => Err(ParseError::expected_input()),
                }
            }

            fn to_value(&self) -> Value {
                Value::Number((*self).into())
            }
        }

        )*
    };
}

impl_type_for_integers!((i8, "int8"), (i16, "int16"), (i32, "int32"), (i64, "int64"));
