use std::string::ToString;
use std::sync::Mutex;
use crate::message::entities::{ErrorResponseBody, LoginRequestBody, LoginResponseBody, RegisterRequestBody, RegisterResponseBody, RequestBody, ResponseBody};
use once_cell::sync::Lazy;
use pingora::http::StatusCode;

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

    fn parse_request_body<T: RequestBody>(data: &Vec<u8>) -> (Option<Box<T>>, Option<ErrorResponseBody>, StatusCode) {
        match T::from_bytes(data.clone()) {
            Ok(body) => (Some(body), None, StatusCode::OK),
            Err(e) => {
                (None, Some(ErrorResponseBody {
                    error: e.to_string(),
                }), StatusCode::BAD_REQUEST)
            }
        }
    }

    pub fn handle_login(&self, data: &Vec<u8>) -> (Option<Vec<u8>>, StatusCode) {

        let (body, error, status) = Self::parse_request_body::<LoginRequestBody>(data);
        if status != StatusCode::OK {
            return (error.map(|e| e.to_bytes()), status);
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        let users = USERS.lock().unwrap(); // Lock the USERS vector to ensure thread safety

        // Validate username and password
        if !users.contains(&(request_body.username, request_body.password)) {
            return (Some(ErrorResponseBody {
                error: "Invalid username or password".to_string(),
            }.to_bytes()), StatusCode::UNAUTHORIZED)
        }

        (Some((LoginResponseBody {
            token: "jwt_token".to_string(), //todo create a real jwt token
        }).to_bytes()), StatusCode::OK)
    }

    pub fn handle_register(&self, data: &Vec<u8>) -> (Option<Vec<u8>>, StatusCode) {
        let (body, error, status) = Self::parse_request_body::<RegisterRequestBody>(data);
        if status != StatusCode::OK {
            return (error.map(|e| e.to_bytes()), status);
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        let mut users = USERS.lock().unwrap(); // Lock the USERS vector to ensure thread safety

        // Check if the username already exists
        if users.iter().any(|(user, _)| user == &request_body.username) {
            let response_body = ErrorResponseBody {
                error: "Username already exists".to_string(),
            };
            return (Some(response_body.to_bytes()), StatusCode::BAD_REQUEST)
        }

        // Save new user in memory
        users.push((request_body.username, request_body.password));

        (Some((RegisterResponseBody {
            success: true,
            message: "User registered successfully".to_string(),
        }).to_bytes()), StatusCode::OK)
    }
}