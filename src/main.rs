#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![feature(async_closure)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server, StatusCode};
use log::{debug, info, trace};
//
mod errors;
mod models;
mod routes;
mod schema;
mod utils;
use errors::ServiceError;
use routes::middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "notajobboard_api_rs=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::init();

    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, ServiceError>(service_fn(move |req| {
                async {
                    trace!("Incoming request: {:?}", req);

                    let processRes = match req.uri().path() {
                        path if path.starts_with("/auth") => {
                            routes::user_login_logout::handle(req).await
                        }
                        path if path.starts_with("/users/check") => {
                            routes::users_check::handle(req).await
                        }
                        path if path.starts_with("/users/register") => {
                            routes::users_register_::handle(req).await
                        }
                        path if path.starts_with("/protected") => {
                            routes::protected::handle(req).await
                        }
                        "/posts" => routes::_posts::handle(req).await,

                        _ => Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::empty())?),
                    };

                    let resp = middleware::errToResp(processRes);
                    let resp = middleware::accessControlHeader(resp);

                    debug!("RESPONSE: {:?}", resp);
                    Ok::<_, ServiceError>(resp)
                }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);
    info!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
