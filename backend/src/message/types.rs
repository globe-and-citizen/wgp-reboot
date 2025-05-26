use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::message::{RequestBody, ResponseBody};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseBody {
    pub error: String,
}

impl ResponseBody for ErrorResponseBody {}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMetadata {
    pub username: String,
    pub title: String,
    pub avatar: String,
    pub bio: String,
    pub email: String,
    pub location: String,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProfileResponse {
    pub metadata: UserMetadata,
}

impl ResponseBody for GetProfileResponse {}