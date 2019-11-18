use diesel::prelude::*;
use hyper::{Request,Response,Body,StatusCode};
use futures_util::try_stream::TryStreamExt;

//
use crate::models::User;
use crate::errors::ServiceError;
use crate::utils::pass_hash::{hash_password, create_token};



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

    use crate::schema::users::dsl::users;
    let hashed_pass: String = hash_password(&registerUserIn.password)?;
    let user = User::from_details(registerUserIn.email, hashed_pass);
    let inserted_user: User = diesel::insert_into(users).values(&user).get_result(&db_conn)?;
    
    Ok(inserted_user.into())
}

async fn make_response(user: User) -> Result<Response<Body>, ServiceError> {
    let body = serde_json::json!({"user": &user.email}).to_string();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", format!("token={}", create_token(user.email)?))
        .header("Access-Control-Allow-Credentials", "true") // WATCH OUT SECURITY ISSUE
        .body(Body::from(body))?
    )
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let registerUserIn = parse_req(req).await?;
    let got_user = process(registerUserIn).await?;
    make_response(got_user).await
}