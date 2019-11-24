use hyper::{Body, Request};
//
use crate::errors::ServiceError;
use crate::utils::pass_hash::AuthnToken;

pub fn tokenAuthn(req: &Request<Body>) -> Result<AuthnToken, ServiceError> {
    let optToken = match req.headers().get("cookie") {
        Some(headerValue) => {
            let valueSplit = headerValue
                .to_str()
                .unwrap()
                .split("=")
                .collect::<Vec<&str>>();
            let rhs = valueSplit.get(1).unwrap().to_owned();
            Some(rhs)
        }
        None => None,
    };
    match optToken {
        Some(token_str) => {
            let token = AuthnToken::from_str(token_str)?;
            match token.verify() {
                Ok(_) => return Ok(token),
                Err(_) => return Err(ServiceError::Unauthorized),
            }
        }
        None => return Err(ServiceError::Unauthorized),
    }
}
