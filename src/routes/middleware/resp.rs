use hyper::{Body, Response};
//
use crate::errors::ServiceError;

pub fn accessControlHeaders(resp: Response<Body>) -> Response<Body> {
    let (mut parts, body) = resp.into_parts();
    parts.headers.insert(
        "Access-Control-Allow-Origin",
        "http://localhost:3000".parse().unwrap(),
    ); // WATCH OUT SECURITY ISSUE
    parts
        .headers
        .insert("Access-Control-Allow-Credentials", "true".parse().unwrap()); // WATCH OUT SECURITY ISSUE
    Response::from_parts(parts, body)
}

pub fn errToResp(resResp: Result<Response<Body>, ServiceError>) -> Response<Body> {
    match resResp {
        Ok(resp) => resp,
        Err(err) => err.into_resp(),
    }
}
