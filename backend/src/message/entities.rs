use std::fmt::Debug;
use serde::{Deserialize, Serialize};

pub(crate) trait RequestBody: Serialize + for<'de> Deserialize<'de> + Debug {
    fn from_bytes(bytes: Vec<u8>) -> Box<Self> {
        serde_json::from_slice(&bytes).unwrap()
    }
}

pub(crate) trait ResponseBody: Serialize + for<'de> Deserialize<'de> + Debug {
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomRequestBody {
    pub data: String,
}

impl RequestBody for CustomRequestBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomResponseBody {
    pub result: String,
}

impl ResponseBody for CustomResponseBody {}