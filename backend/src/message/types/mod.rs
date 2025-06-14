use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod other;

pub trait ResponseBodyTrait: Serialize + for<'de> Deserialize<'de> + Debug {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

pub trait RequestBodyTrait: Serialize + for<'de> Deserialize<'de> + Debug {
    fn from_bytes(bytes: Vec<u8>) -> Result<Box<Self>, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NTorEncryptMessage {
    pub nonce: [u8; 12],
    pub encrypted: Vec<u8>
}

impl ResponseBodyTrait for NTorEncryptMessage {}
impl RequestBodyTrait for NTorEncryptMessage {}

