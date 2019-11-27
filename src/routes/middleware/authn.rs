use headers::{Cookie, HeaderMapExt};
use hyper::{Body, Request};
//
use crate::errors::ServiceError;
use crate::utils::pass_hash::AuthnToken;

pub fn tokenAuthn(req: &Request<Body>) -> Result<AuthnToken, ServiceError> {
    let cookies = req.headers().typed_get::<Cookie>().expect("oh no");
    // dbg!(cookies.get("adminer_sid"));
    // let optTokenKV = match req.headers().get("cookie") {
    //     None => None,
    //     Some(headerValues) => {
    //         headerValues
    //             .to_str()?
    //             .split("; ")
    //             .map(|headerKV| headerKV.splitn(2, "=").collect::<Vec<&str>>())
    //             // if split.len() != 2 {
    //             //     return Err(ServiceError::new_bad_request("invalid cookie header"));
    //             // }
    //             // split[0] == "token"
    //             .find(|kvSplit| kvSplit[0] == "token")
    //         //
    //         // .map(|i| Ok((split[0], split[1])))
    //         // .take_while(Result::is_ok)
    //         // .map(Result::unwrap)
    //         // .find(|(key, _)| *key == "token");
    //         // .take(1);
    //         // .collect::<Vec<&str>>();
    //     }
    // };
    // match optTokenKV {
    //     None => return Err(ServiceError::Unauthorized),
    //     Some(tokenKV) => {
    //         if tokenKV.len() != 2 {
    //             return Err(ServiceError::new_bad_request("invalid cookie header"));
    //         }
    //     }
    // };
    // let optToken = match req.headers().get("cookie") {
    //     Some(headerValue) => {
    //         dbg!(&headerValue);
    //         let valueSplit = headerValue
    //             .to_str()
    //             .unwrap()
    //             .split("=")
    //             .collect::<Vec<&str>>();
    //         let rhs = valueSplit.get(1).unwrap().to_owned();
    //         Some(rhs)
    //     }
    //     None => None,
    // };
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
