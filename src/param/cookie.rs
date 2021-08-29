use std::collections::HashMap;

use poem::{Error, Request, Result};

use crate::{types::Type, ParseRequestError};

pub fn parse_from_cookie<T: Type>(
    name: &str,
    request: &Request,
    _query: &HashMap<String, String>,
) -> Result<T> {
    let cookie = request.cookie().get(name);
    Ok(
        T::parse_from_str(cookie.as_ref().map(|cookie| cookie.value())).map_err(|err| {
            Error::bad_request(ParseRequestError::ParseParam {
                name: name.to_string(),
                reason: err.into_message(),
            })
        })?,
    )
}
