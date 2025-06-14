use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::message::types::RequestBodyTrait;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestBody {
    pub username: String,
    pub password: String, // todo change to hash
}
impl RequestBodyTrait for LoginRequestBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequestBody {
    pub username: String,
    pub password: String, // todo change to hash
}
impl RequestBodyTrait for RegisterRequestBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct NTorInitRequestBody {
    pub public_key: Vec<u8>
}
impl RequestBodyTrait for NTorInitRequestBody {}