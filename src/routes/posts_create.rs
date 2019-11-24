use diesel::prelude::*;
use futures_util::try_stream::TryStreamExt;
use hyper::{Body, Request, Response, StatusCode};
//
use super::middleware::tokenAuthn;
use crate::errors::ServiceError;
use crate::models::post::{NewPost, Post};
use crate::schema::posts;
use crate::utils::db_conn_pool;
use crate::utils::pass_hash::AuthnToken;

#[derive(Debug, Deserialize)]
pub struct CreatePostIn {
    pub title: String,
    pub body: String,
}
pub struct Ctx {
    pub token: AuthnToken,
}

async fn parse_req(req: Request<Body>) -> Result<CreatePostIn, ServiceError> {
    // extract createPostIn from body
    let body = req.into_body().try_concat().await?;
    let createPostIn = serde_json::from_slice::<CreatePostIn>(&body)?;
    Ok(createPostIn)
}

async fn process<'a>(ctx: Ctx, createPostIn: CreatePostIn) -> Result<Post, ServiceError> {
    let db_conn = db_conn_pool::get_db_conn()?;

    let new_post = NewPost {
        title: &createPostIn.title,
        body: &createPostIn.body,
        author_id: ctx.token.claims.userID,
    };

    let inserted_post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&db_conn)?;

    Ok(inserted_post)
}

async fn make_response(post: Post) -> Result<Response<Body>, ServiceError> {
    // let body = serde_json::json!({ "post": &post }).to_string();
    let body = serde_json::to_string(&post)?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Credentials", "true") // WATCH OUT SECURITY ISSUE
        .body(Body::from(body))?)
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let ctx = Ctx {
        token: tokenAuthn(&req)?,
    };
    //
    let createPostIn = parse_req(req).await?;
    let post = process(ctx, createPostIn).await?;
    make_response(post).await
}
