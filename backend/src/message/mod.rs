use std::fmt::Debug;
use serde::{Deserialize, Serialize};

mod types;
pub mod handler;
mod error;

pub trait RequestBody: Serialize + for<'de> Deserialize<'de> + Debug {
    fn from_bytes(bytes: Vec<u8>) -> Result<Box<Self>, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }
}

pub trait ResponseBody: Serialize + for<'de> Deserialize<'de> + Debug {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}
