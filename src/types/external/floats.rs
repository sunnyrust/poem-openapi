use serde_json::{Number, Value};

use crate::types::{DataType, ParseError, ParseResult, Type};

macro_rules! impl_type_for_floats {
    ($(($ty:ty, $format:literal)),*) => {
        $(
        impl Type for $ty {
            const DATA_TYPE: DataType = DataType::Normal {
                ty: "number",
                format: Some($format),
            };

            fn parse(value: Option<Value>) -> ParseResult<Self> {
                if let Some(Value::Number(n)) = value {
                    let n = n
                        .as_f64()
                        .ok_or_else(|| ParseError::from("invalid number"))?;
                    Ok(n as Self)
                } else {
                    Err(ParseError::expected_type(value.unwrap_or_default()))
                }
            }

            fn parse_from_str(value: Option<&str>) -> ParseResult<Self> {
                match value {
                    Some(value) => value.parse().map_err(ParseError::custom),
                    None => Err(ParseError::expected_input()),
                }
            }

            fn to_value(&self) -> Value {
                Value::Number(Number::from_f64(*self as f64).unwrap())
            }
        }

        )*
    };
}

impl_type_for_floats!((f32, "float32"), (f64, "float64"));
