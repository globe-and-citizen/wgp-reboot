use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const JWT_SECRET: &[u8; 18] = b"this is wgp secret";

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn get_username(&self) -> String {
        self.sub.clone()
    }
}

pub fn create_jwt_token(username: String) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ).unwrap()
}

pub fn verify_jwt_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
}

pub fn new_nTor_session_id() -> String {
    Uuid::new_v4().to_string()
}
