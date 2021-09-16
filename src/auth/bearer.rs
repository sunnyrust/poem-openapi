use poem::Request;
use typed_headers::{AuthScheme, Authorization, HeaderMapExt};

use crate::{auth::BearerAuthorization, ParseRequestError};

/// Used to extract the token68 from the request.
pub struct Bearer {
    /// token
    pub token: String,
}

impl BearerAuthorization for Bearer {
    fn from_request(req: &Request) -> Result<Self, ParseRequestError> {
        if let Some(auth) = req.headers().typed_get::<Authorization>().ok().flatten() {
            if auth.0.scheme() == &AuthScheme::BEARER {
                if let Some(token68) = auth.token68() {
                    return Ok(Bearer {
                        token: token68.as_str().to_string(),
                    });
                }
            }
        }

        Err(ParseRequestError::Authorization)
    }
}
