use std::{fmt::Display, marker::PhantomData};

use serde_json::Value;

use super::Type;

/// An error parsing an schema.
///
/// This type is generic over T as it uses T's type name when converting to a
/// regular error.
#[derive(Debug)]
pub struct ParseError<T> {
    message: String,
    phantom: PhantomData<T>,
}

impl<T: Type, E: Display> From<E> for ParseError<T> {
    fn from(error: E) -> Self {
        Self::custom(error)
    }
}

impl<T: Type> ParseError<T> {
    fn new(message: String) -> Self {
        Self {
            message,
            phantom: PhantomData,
        }
    }

    /// The expected input type did not match the actual input type.
    #[must_use]
    pub fn expected_type(actual: Value) -> Self {
        Self::new(format!(
            r#"Expected input type "{}", found {}."#,
            T::DATA_TYPE,
            actual
        ))
    }

    /// Type A expects an input value.
    #[must_use]
    pub fn expected_input() -> Self {
        Self::new(format!(
            r#"Type "{}" expects an input value."#,
            T::DATA_TYPE
        ))
    }

    /// This type does not support parsing from string.
    #[must_use]
    pub fn not_support_parsing_from_string() -> Self {
        Self::custom("not support parsing from string")
    }

    /// A custom error message.
    ///
    /// Any type that implements `Display` is automatically converted to this if
    /// you use the `?` operator.
    #[must_use]
    pub fn custom(msg: impl Display) -> Self {
        Self::new(format!(r#"failed to parse "{}": {}"#, T::DATA_TYPE, msg))
    }

    /// Propagate the error message to a different type.
    pub fn propagate<U: Type>(self) -> ParseError<U> {
        if T::DATA_TYPE != U::DATA_TYPE {
            ParseError::new(format!(
                r#"{} (occurred while parsing "{}")"#,
                self.message,
                U::DATA_TYPE
            ))
        } else {
            ParseError::new(self.message)
        }
    }

    /// Consume this error and convert it into a message.
    pub fn into_message(self) -> String {
        self.message
    }
}

/// An error parsing a value of type `T`.
pub type ParseResult<T> = Result<T, ParseError<T>>;
