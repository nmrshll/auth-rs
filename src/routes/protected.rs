// use diesel::prelude::*;
// use futures_util::try_stream::TryStreamExt;
use hyper::{Body, Request, Response};

//
use super::middleware::tokenAuthn;
use crate::errors::ServiceError;

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let token = tokenAuthn(&req)?;
    // let checkUserIn = parse_req(req).await?;
    // let got_user = process(checkUserIn).await?;
    // make_response(got_user).await
    Ok(Response::new(req.into_body()))
}
