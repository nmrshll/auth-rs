use diesel::prelude::*;
use futures_util::try_stream::TryStreamExt;
use hyper::{Body, Request, Response, StatusCode};

//
use crate::errors::ServiceError;
use crate::models::User;
use crate::utils::pass_hash::decode_token;

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let optToken = match req.headers().get("cookie") {
        Some(headerValue) => {
            let valueSplit = headerValue
                .to_str()
                .unwrap()
                .split("=")
                .collect::<Vec<&str>>();
            let one = valueSplit.get(1).unwrap().to_owned();
            Some(one)
        }
        None => None,
    };
    match optToken {
        Some(token) => {
            let decoded = decode_token(&token)?;
            dbg!(&decoded);
        }
        None => return Err(ServiceError::Unauthorized),
    }

    // let checkUserIn = parse_req(req).await?;
    // let got_user = process(checkUserIn).await?;
    // make_response(got_user).await

    Ok(Response::new(req.into_body()))
}
