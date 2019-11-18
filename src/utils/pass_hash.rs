// use bcrypt::{hash,DEFAULT_COST};
use chrono::{Duration,Local};
use jsonwebtoken::{decode,encode,Header,Validation};
use argonautica::{Hasher, Verifier};

//

use crate::errors::ServiceError;


lazy_static::lazy_static! {
    // WARNING: pass secret via env
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "f7sigfh2dsjk56fghdj4g,fhjd62kg".repeat(8));
}


pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SlimUser {
//     pub email: String,
// }

// impl From<User> for SlimUser {
//     fn from(user: User) -> Self {
//         SlimUser { email: user.email }
//     }
// }

// JWT claim
#[derive(Debug, Serialize,Deserialize)]
struct Claim {
    iss:String,
    sub:String,
    // issued_at
    iat:i64,
    exp:i64,
    email:String,
}
// convert email to token
impl Claim {
    fn with_email(email:&str) -> Self {
        Claim {
            iss:"localhost".into(),
            sub: "auth".into(),
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() +Duration::hours(24)).timestamp(),
        }
    }
}

pub type UserEmail = String;
impl From<Claim> for UserEmail {
    fn from(claims: Claim) -> Self {
        claims.email
    }
}

pub fn create_token(userEmail: UserEmail) -> Result<String, ServiceError> {
    let claims = Claim::with_email(userEmail.as_str());
    encode(&Header::default(),&claims, SECRET_KEY.as_ref()).map_err(|_err| ServiceError::InternalServerError)
}

pub fn decode_token(token:&str) -> Result<UserEmail, ServiceError> {
    decode::<Claim>(token, SECRET_KEY.as_ref(), &Validation::default()).map(|data| Ok(data.claims.into())).map_err(|_err| ServiceError::Unauthorized)?
}

// // this could be implemented using lazy_static crate
// fn SECRET_KEY -> String {
//     std::env::var("JWT_SECRET").unwrap_or_else(|_| "my secret".into())
// }