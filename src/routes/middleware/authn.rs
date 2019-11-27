use headers::{Cookie, HeaderMapExt};
use hyper::{Body, Request};
//
use crate::errors::ServiceError;
use crate::utils::pass_hash::AuthnToken;

pub fn tokenAuthn(req: &Request<Body>) -> Result<AuthnToken, ServiceError> {
    let cookies = match req.headers().typed_get::<Cookie>() {
        None => return Err(ServiceError::Unauthorized),
        Some(cookies) => cookies,
    };
    match cookies.get("token") {
        Some(token_str) => {
            dbg!(&token_str);
            let token = AuthnToken::from_str(token_str)?;
            match token.verify() {
                Ok(_) => return Ok(token),
                Err(_) => return Err(ServiceError::Unauthorized),
            }
        }
        None => return Err(ServiceError::Unauthorized),
    }
}
