use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub(crate) trait RequestBody: Serialize + for<'de> Deserialize<'de> + Debug {
    fn from_bytes(bytes: Vec<u8>) -> Result<Box<Self>, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }
}

pub(crate) trait ResponseBody: Serialize + for<'de> Deserialize<'de> + Debug {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestBody {
    pub username: String,
    pub password: String, // todo change to hash
}

impl RequestBody for LoginRequestBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponseBody {
    pub token: String,
}

impl ResponseBody for LoginResponseBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequestBody {
    pub username: String,
    pub password: String, // todo change to hash
}

impl RequestBody for RegisterRequestBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponseBody {
    pub success: bool,
    pub message: String,
}
impl ResponseBody for RegisterResponseBody {}