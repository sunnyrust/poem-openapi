use std::collections::HashMap;

use poem::{Error, Request, Result};

use crate::{types::Type, ParseRequestError};

pub fn parse_from_header<T: Type>(
    name: &str,
    request: &Request,
    _query: &HashMap<String, String>,
) -> Result<T> {
    let value = request
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok());
    Ok(T::parse_from_str(value).map_err(|err| {
        Error::bad_request(ParseRequestError::ParseParam {
            name: name.to_string(),
            reason: err.into_message(),
        })
    })?)
}
