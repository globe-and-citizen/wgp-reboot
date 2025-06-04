use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub fn create_jwt_token(username: String, jwt_secret: [u8; 32]) -> String {
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
        &EncodingKey::from_secret(&jwt_secret),
    ).unwrap()
}

pub fn verify_jwt_token(token: &str, jwt_secret: [u8; 32]) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&jwt_secret),
        &Validation::default(),
    )
}

pub fn new_nTor_session_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn vec_to_json(vec: Vec<u8>) -> String {
    serde_json::to_string(&vec).unwrap()
}

pub fn json_to_vec(json: &str) -> Vec<u8> {
    serde_json::from_str(json).unwrap()
}

pub fn string_to_array32(s: String) -> Option<[u8; 32]> {
    let bytes = s.into_bytes();
    if bytes.len() == 32 {
        Some(bytes.try_into().unwrap())
    } else {
        None
    }
}
