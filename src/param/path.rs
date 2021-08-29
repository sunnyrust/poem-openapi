use std::collections::HashMap;

use poem::{Error, Request, Result};

use crate::{types::Type, ParseRequestError};

pub fn parse_from_path<T: Type>(
    name: &str,
    request: &Request,
    _query: &HashMap<String, String>,
) -> Result<T> {
    Ok(T::parse_from_str(request.path_param(name)).map_err(|err| {
        Error::bad_request(ParseRequestError::ParseParam {
            name: name.to_string(),
            reason: err.into_message(),
        })
    })?)
}
