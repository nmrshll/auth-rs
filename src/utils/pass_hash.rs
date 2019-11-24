use argonautica::{Hasher, Verifier};
use bytes::Bytes;
use chrono::{Duration, Local};
use crypto::blake2b::Blake2b;
use crypto::digest::Digest;
// use jsonwebtoken::{decode, encode, Header, Validation};
use rand::rngs::OsRng;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey, Signature};
//
// use arrayvec::ArrayVec;
// use log::info;
// use rand_test::thread_rng;
// use secp256k1_test::Secp256k1;
use std::fs;
// use std::sync::Mutex;

//

use crate::errors::ServiceError;

const KEYS_FOLDER: &'static str = "./.cache/keys"; // WARNING pass via configMap
lazy_static::lazy_static! {
    // WARNING: pass secret via env
    pub static ref SECRET_KEY_HASH: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "f7sigfh2dsjk56fghdj4g,fhjd62kg".repeat(8));
    //
    pub static ref SECP: Secp256k1<All> = Secp256k1::new();
    //
    pub static ref KEYPAIR_SIGN:KeyPair = KeyPair::from_file_or_new("token_sign").unwrap();
    pub static ref PRIVKEY_SIGN:SecretKey = KEYPAIR_SIGN.privkey;
    pub static ref PUBKEY_SIGN:PublicKey = KEYPAIR_SIGN.pubkey;
    //
}

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY_HASH.as_str())
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
        .with_secret_key(SECRET_KEY_HASH.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}

//

//////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,
    pub exp: i64,
    pub userID: i64,
}
impl Claims {
    fn with_userId(userId: i64) -> Self {
        Self {
            userID: userId,
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
    fn hash(&self) -> [u8; 32] {
        let mut ret = [0u8; 32];
        let mut hasher = Blake2b::new(32);
        hasher.input(&self.userID.to_be_bytes());
        hasher.input(&self.iat.to_be_bytes());
        hasher.input(&self.exp.to_be_bytes());
        hasher.result(&mut ret);
        ret
    }
    fn into_secp_message(&self) -> Message {
        Message::from_slice(&self.hash()).expect("32 bytes")
    }
    fn sign(self) -> Result<AuthnToken, ServiceError> {
        let sig = SECP.sign(&self.into_secp_message(), &PRIVKEY_SIGN);
        Ok(AuthnToken { claims: self, sig })
    }
}
#[derive(Debug)]
pub struct AuthnToken {
    pub claims: Claims,
    pub sig: Signature,
}
impl AuthnToken {
    pub fn from_userId(userId: i64) -> Result<AuthnToken, ServiceError> {
        Claims::with_userId(userId).sign()
    }
    pub fn verify(&self) -> Result<(), ServiceError> {
        if self.claims.exp < Local::now().timestamp() {
            return Err(ServiceError::Unauthorized);
        }
        match SECP.verify(&self.claims.into_secp_message(), &self.sig, &PUBKEY_SIGN) {
            Ok(()) => Ok(()),
            Err(_e) => Err(ServiceError::Unauthorized),
        }
    }
    pub fn to_string(&self) -> String {
        let mut b = Bytes::new();
        b.extend_from_slice(&self.claims.iat.to_be_bytes());
        b.extend_from_slice(&self.claims.exp.to_be_bytes());
        b.extend_from_slice(&self.claims.userID.to_be_bytes());
        b.extend_from_slice(&self.sig.serialize_compact());
        base64::encode(&b)
    }
    pub fn from_str(token: &str) -> Result<AuthnToken, ServiceError> {
        let bytes = base64::decode(&token)?;
        //
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[0..8]);
        let iat: i64 = i64::from_be_bytes(buf);
        buf.copy_from_slice(&bytes[8..16]);
        let exp: i64 = i64::from_be_bytes(buf);
        buf.copy_from_slice(&bytes[16..24]);
        let userID: i64 = i64::from_be_bytes(buf);
        //
        let sig: Signature = Signature::from_compact(&bytes[24..])?;

        Ok(AuthnToken {
            claims: Claims { iat, exp, userID },
            sig,
        })
    }
}

////////////////////////////

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyPair {
    pub pubkey: PublicKey,
    privkey: SecretKey,
}
impl KeyPair {
    // generate new keys
    fn new() -> Self {
        let mut rng = OsRng::new().expect("OsRng");
        let (privkey, pubkey) = SECP.generate_keypair(&mut rng);
        Self { pubkey, privkey }
    }
    fn to_file(&self, keyfile: &str) -> Result<&Self, Box<dyn std::error::Error + Send + Sync>> {
        fs::create_dir_all(KEYS_FOLDER)?;
        fs::write(keyfile, serde_json::to_string(self)?).expect("Unable to write file");
        Ok(self)
    }
    fn from_file(keyfile: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content_string = fs::read_to_string(keyfile)?;
        Ok(serde_json::from_str(&content_string)?)
    }
    fn from_file_or_new(keyfile: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let keyfile = format!("{}/{}", KEYS_FOLDER, keyfile);
        match Self::from_file(&keyfile) {
            Ok(identity) => Ok(identity),
            Err(_err) => {
                let newWallet = Self::new();
                newWallet.to_file(&keyfile)?;
                Ok(newWallet)
            }
        }
    }
}

//////////////////////

// JWT claim
// #[derive(Debug, Serialize, Deserialize)]
// struct Claim {
//     iss: String,
//     sub: String,
//     // issued_at
//     iat: i64,
//     exp: i64,
//     email: String,
// }
// // convert email to token
// impl Claim {
//     fn with_email(email: &str) -> Self {
//         Claim {
//             iss: "localhost".into(),
//             sub: "auth".into(),
//             email: email.to_owned(),
//             iat: Local::now().timestamp(),
//             exp: (Local::now() + Duration::hours(24)).timestamp(),
//         }
//     }
// }

// pub type UserEmail = String;
// impl From<Claim> for UserEmail {
//     fn from(claims: Claim) -> Self {
//         claims.email
//     }
// }

// pub fn create_token(userEmail: UserEmail) -> Result<String, ServiceError> {
//     let claims = Claim::with_email(userEmail.as_str());
//     encode(&Header::default(), &claims, SECRET_KEY.as_ref())
//         .map_err(|_err| ServiceError::InternalServerError)
// }

// pub fn decode_token(token: &str) -> Result<UserEmail, ServiceError> {
//     decode::<Claim>(token, SECRET_KEY.as_ref(), &Validation::default())
//         .map(|data| Ok(data.claims.into()))
//         .map_err(|_err| ServiceError::Unauthorized)?
// }
