use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::message::types::other::{Poem, UserMetadata};
use crate::message::types::ResponseBodyTrait;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseBody {
    pub error: String,
}

impl ResponseBodyTrait for ErrorResponseBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponseBody {
    pub token: String,
}

impl ResponseBodyTrait for LoginResponseBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponseBody {
    pub success: bool,
    pub message: String,
}

impl ResponseBodyTrait for RegisterResponseBody {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProfileResponse {
    pub metadata: UserMetadata,
}

impl ResponseBodyTrait for GetProfileResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPoemResponse {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
}

impl ResponseBodyTrait for GetPoemResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPoemsResponse {
    pub(crate) poems: Box<[Poem]>,
}

impl ResponseBodyTrait for GetPoemsResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetImageResponse {
    pub id: i32,
    pub title: String,
    pub file_name: String,
    pub content: Vec<u8>,
}

impl ResponseBodyTrait for GetImageResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetImagesResponse {
    pub images: Box<[GetImageResponse]>,
}

impl ResponseBodyTrait for GetImagesResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct NTorInitResponse {
    pub public_key: Vec<u8>,
    pub t_hash: Vec<u8>,
    pub session_id: String,
    pub static_public_key: Vec<u8>, // fixme this field can be removed
    pub server_id: String
}

impl ResponseBodyTrait for NTorInitResponse {}
