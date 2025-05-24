use std::string::ToString;
use std::sync::Mutex;
use log::debug;
use crate::message::entities::{LoginRequestBody, LoginResponseBody, RegisterRequestBody, RegisterResponseBody, RequestBody, ResponseBody};
use crate::message::error::MessageHandlerError;
use once_cell::sync::Lazy;

static USERS: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(Vec::from(&[
    ("tester".to_string(), "1234".to_string()),
])));

pub struct MessageHandler {
    // whatever can be added later as needed
}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {}
    }

    fn parse_request_body<T: RequestBody>(data: &Vec<u8>) -> Result<Box<T>, pingora::BError> {
        match T::from_bytes(data.clone()) {
            Ok(body) => {
                debug!("Request body: {:?}", body);
                Ok(body)
            },
            Err(e) => Err(MessageHandlerError::ParseBodyError(e.into()).to_pingora_error()),
        }
    }

    pub fn handle_login(&self, data: &Vec<u8>, ) -> Result<Option<Vec<u8>>, pingora::BError> {

        let request_body = Self::parse_request_body::<LoginRequestBody>(data)?;

        let users = USERS.lock().unwrap(); // Lock the USERS vector to ensure thread safety
        // Validate username and password
        users.contains(&(request_body.username, request_body.password))
            .then_some(())
            .ok_or_else(|| MessageHandlerError::InvalidData("Invalid username or password").to_pingora_error())?;

        Ok(Some((LoginResponseBody {
            token: "jwt_token".to_string(), //todo create a real jwt token
        }).to_bytes()))
    }

    pub fn handle_register(&self, data: &Vec<u8>, ) -> Result<Option<Vec<u8>>, pingora::BError> {

        let request_body = Self::parse_request_body::<RegisterRequestBody>(data)?;

        let mut users = USERS.lock().unwrap(); // Lock the USERS vector to ensure thread safety

        // Check if the username already exists
        if users.iter().any(|(user, _)| user == &request_body.username) {
            return Err(MessageHandlerError::InvalidData("Username already exists").to_pingora_error());
        }

        // Save new user in memory
        users.push((request_body.username, request_body.password));

        Ok(Some((RegisterResponseBody {
            success: true,
            message: "User registered successfully".to_string(),
        }).to_bytes()))
    }
}