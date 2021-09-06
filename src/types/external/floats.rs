use serde_json::{Number, Value};

use crate::{
    registry::MetaSchemaRef,
    types::{ParseError, ParseResult, Type, TypeName},
};

macro_rules! impl_type_for_floats {
    ($(($ty:ty, $format:literal)),*) => {
        $(
        impl Type for $ty {
            const NAME: TypeName = TypeName::Normal {
                ty: "number",
                format: Some($format),
            };

            fn schema_ref() -> MetaSchemaRef {
                MetaSchemaRef::Inline(Self::NAME.into())
            }

            fn parse(value: Value) -> ParseResult<Self> {
                if let Value::Number(n) = value {
                    let n = n
                        .as_f64()
                        .ok_or_else(|| ParseError::from("invalid number"))?;
                    Ok(n as Self)
                } else {
                    Err(ParseError::expected_type(value))
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
