// use diesel::prelude::*;
use futures_util::try_stream::TryStreamExt;
use hyper::{Body, Request, Response, StatusCode};

//
use crate::errors::ServiceError;
use crate::models::{NewUser, User};
use crate::utils::pass_hash::AuthnToken;

#[derive(Debug, Deserialize)]
pub struct RegisterUserIn {
    pub email: String,
    pub password: String,
}

async fn parse_req(req: Request<Body>) -> Result<RegisterUserIn, ServiceError> {
    // extract registerUserIn from body
    let body = req.into_body().try_concat().await?;
    let registerUserIn = serde_json::from_slice::<RegisterUserIn>(&body)?;
    Ok(registerUserIn)
}

async fn process(registerUserIn: RegisterUserIn) -> Result<User, ServiceError> {
    let db_conn = crate::utils::db_conn_pool::get_db_conn()?;

    let newUser = NewUser::from_credentials(&registerUserIn.email, &registerUserIn.password)?;
    let user = newUser.insert(&db_conn)?;

    Ok(user)
}

async fn make_response(user: User) -> Result<Response<Body>, ServiceError> {
    let body = serde_json::json!({"user": &user.email}).to_string();
    let token = AuthnToken::from_userId(user.id)?.to_string();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", format!("token={}", token))
        .header("Access-Control-Allow-Credentials", "true") // WATCH OUT SECURITY ISSUE
        .body(Body::from(body))?)
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let registerUserIn = parse_req(req).await?;
    let got_user = process(registerUserIn).await?;
    make_response(got_user).await
}
