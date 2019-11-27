use diesel::prelude::*;
use futures_util::try_stream::TryStreamExt;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::error::Error;

//
use crate::errors::ServiceError;
use crate::models::User;
use crate::utils::db_conn_pool;
use crate::utils::pass_hash::{self, AuthnToken};

mod login {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct UserAuthIn {
        pub email: String,
        pub password: String,
    }

    pub async fn parse_req(req: Request<Body>) -> Result<UserAuthIn, ServiceError> {
        // extract auth data from JSON
        let body = req.into_body().try_concat().await?;
        match serde_json::from_slice::<UserAuthIn>(&body) {
            Ok(userAuthIn) => Ok(userAuthIn),
            Err(err) => Err(ServiceError::BadRequest {
                ctx: err.description().into(),
                source: Box::new(err),
            }),
        }
    }

    pub async fn process(userAuthIn: UserAuthIn) -> Result<User, ServiceError> {
        let db_conn = db_conn_pool::get_db_conn()?;

        use crate::schema::users::dsl::{email, users};
        let mut foundUsers = users
            .filter(email.eq(&userAuthIn.email))
            .load::<User>(&db_conn)?;

        if let Some(user) = foundUsers.pop() {
            if let Ok(matching) = pass_hash::verify(&user.hash_pass, &userAuthIn.password) {
                if matching {
                    return Ok(user.into());
                }
            }
        }
        Err(ServiceError::Unauthorized)
    }

    pub async fn make_response(user: User) -> Result<Response<Body>, ServiceError> {
        let body = serde_json::json!({"user": &user.email}).to_string();
        let token = AuthnToken::from_userId(user.id)?.to_string();
        dbg!(&token);

        // set signed cookie with userID
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Set-Cookie", format!("token={};Path=/", token))
            .body(Body::from(body))?)
    }
}

mod getLoggedUser {
    use super::*;

    pub async fn handle(_req: Request<Body>) -> Result<Response<Body>, ServiceError> {
        // get loggedUser from auth cookie
        unimplemented!()
    }
}

mod logout {
    use super::*;

    pub async fn handle(_req: Request<Body>) -> Result<Response<Body>, ServiceError> {
        // Ok response and unset cookie
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())?)
    }
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    match req.method() {
        &Method::GET => {
            // get user info
            getLoggedUser::handle(req).await
        }
        &Method::POST => {
            // login
            let userAuthIn = login::parse_req(req).await?;
            let loggedUser = login::process(userAuthIn).await?;
            login::make_response(loggedUser).await
        }
        &Method::DELETE => {
            // logout
            logout::handle(req).await
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?),
    }
}
