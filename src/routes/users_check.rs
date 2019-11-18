use diesel::prelude::*;
use hyper::{Request,Response,Body,StatusCode};
use futures_util::try_stream::TryStreamExt;

//
use crate::models::User;
use crate::errors::ServiceError;



#[derive(Debug, Deserialize)]
pub struct CheckUserIn {
    pub email: String,
}

async fn parse_req(req: Request<Body>) -> Result<CheckUserIn, ServiceError> {
    // extract checkUserIn from body
    let body = req.into_body().try_concat().await?;
    let checkUserIn = serde_json::from_slice::<CheckUserIn>(&body)?;
    Ok(checkUserIn)
}

async fn process(checkUserIn: CheckUserIn) -> Result<User, ServiceError> {
    let db_conn = crate::utils::db_conn_pool::get_db_conn()?;

    use crate::schema::users::dsl::{email, users};
    let mut foundUsers = users
        .filter(email.eq(&checkUserIn.email))
        .load::<User>(&db_conn)?;

    match foundUsers.len() {
        0 => Err(ServiceError::NotFound),
        1 => Ok(foundUsers.pop().unwrap().into()),
        _ => Err(ServiceError::InternalServerError)
    }
}

async fn make_response(user: User) -> Result<Response<Body>, ServiceError> {
    let body = serde_json::json!({"user": &user.email}).to_string();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))?
    )
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let checkUserIn = parse_req(req).await?;
    let got_user = process(checkUserIn).await?;
    make_response(got_user).await
}