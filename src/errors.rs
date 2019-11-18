use diesel::result::{DatabaseErrorKind, Error as DBError};
use hyper::header::CONTENT_LENGTH;
use hyper::{Body, Response, StatusCode};
use log::error;
use snafu::Snafu;
use std::convert::From;
use std::error::Error;
use uuid::parser::ParseError as UUIDParseError;

#[derive(Debug, Snafu)]
pub enum ServiceError {
    // #[snafu(display("Bad Request: {}", cause))]
    // BadRequest { cause: String },
    #[snafu(display("Bad Request: {}: {}", ctx, source))]
    BadRequest {
        ctx: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[snafu(display("Bad Request: {}", ctx))]
    NewBadRequest { ctx: String },

    #[snafu(display("Internal Server Error"))]
    InternalServerError,

    #[snafu(display("Unauthorized"))]
    Unauthorized,
    #[snafu(display("Not found"))]
    NotFound,

    #[snafu(display("hyper error: {}", hyperError))]
    HyperError { hyperError: hyper::Error },
}
impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ServiceError::NewBadRequest { .. } => StatusCode::BAD_REQUEST,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
            //
            ServiceError::HyperError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn into_resp(&self) -> Response<Body> {
        let error_message = format!("{}", &self);
        Response::builder()
            .header(CONTENT_LENGTH, error_message.len() as u64)
            .status(self.status_code())
            .body(Body::from(error_message))
            .expect("Failed to construct a response")
    }

    // pub fn already_exists(ctx: &'static str) -> Self {
    //     Self::AlreadyExists { ctx }
    // }
    pub fn new_bad_request(ctx: &str) -> Self {
        Self::NewBadRequest {
            ctx: ctx.to_string(),
        }
    }
}

// pub type Result<T, E = Error> = std::result::Result<T, E>;

// we can return early in our handlers if UUID provided by the user is not valid
// we then return a custom message
impl From<UUIDParseError> for ServiceError {
    fn from(_: UUIDParseError) -> ServiceError {
        ServiceError::NewBadRequest {
            ctx: "Invalid UUID".into(),
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // right now we just care about uniqueViolation from diesel
        // but we use a match to map new diesel error types as app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::NewBadRequest {
                        ctx: message.into(),
                    };
                }
                error!("database error: kind: {:?}, info: {:?}", kind, info);
                ServiceError::InternalServerError
            }
            err => {
                error!("{}", err);
                ServiceError::InternalServerError
            }
        }
    }
}
impl From<hyper::Error> for ServiceError {
    fn from(err: hyper::Error) -> ServiceError {
        ServiceError::HyperError { hyperError: err }
    }
}
impl From<hyper::http::Error> for ServiceError {
    fn from(_err: hyper::http::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}
impl From<serde_json::error::Error> for ServiceError {
    fn from(err: serde_json::error::Error) -> Self {
        ServiceError::NewBadRequest {
            ctx: err.description().into(),
        }
    }
}
impl From<r2d2::Error> for ServiceError {
    fn from(_err: r2d2::Error) -> Self {
        ServiceError::InternalServerError
    }
}
// impl From<http::header::value::ToStrError> for ServiceError {
//     fn from(_err: http::header::value::ToStrError) -> Self {
//         ServiceError::InternalServerError
//     }
// }

pub mod mapFns {
    use super::*;
    pub fn intern_err<T, E: std::error::Error>(_: E) -> Result<T, ServiceError> {
        Err(ServiceError::InternalServerError)
    }
}
