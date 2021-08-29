use std::collections::HashMap;

use poem::{Error, Request, Result};

use crate::{types::Type, ParseRequestError};

pub fn parse_from_query<T: Type>(
    name: &str,
    _request: &Request,
    query: &HashMap<String, String>,
) -> Result<T> {
    let value = query.get(name).map(|s| s.as_str());
    Ok(T::parse_from_str(value).map_err(|err| {
        Error::bad_request(ParseRequestError::ParseParam {
            name: name.to_string(),
            reason: err.into_message(),
        })
    })?)
}
