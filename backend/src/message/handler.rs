use std::string::ToString;
use std::sync::Mutex;
use crate::message::types::{ErrorResponseBody, GetProfileResponse, LoginRequestBody, LoginResponseBody, RegisterRequestBody, RegisterResponseBody};
use crate::message::{RequestBody, ResponseBody};
use once_cell::sync::Lazy;
use pingora::http::StatusCode;
use crate::router::types::{ContextTrait, Response};

static USERS: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(Vec::from(&[
    ("tester".to_string(), "1234".to_string()),
])));

const TEMPORARY_JWT_TOKEN: &str = "temporary_jwt_token"; // Placeholder for a real JWT token

pub struct WGPMessageHandler {
    // whatever can be added later as needed
}

impl WGPMessageHandler {
    pub fn new() -> Self {
        WGPMessageHandler {}
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

    pub fn handle_login(&self, ctx: &dyn ContextTrait) -> Response {
        let data = ctx.request_body();
        let (body, error, status) = Self::parse_request_body::<LoginRequestBody>(data);
        if status != StatusCode::OK {
            return Response::new(status, error.map(|e| e.to_bytes()));
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        let users = USERS.lock().unwrap(); // Lock the USERS vector to ensure thread safety

        // Validate username and password
        if !users.contains(&(request_body.username, request_body.password)) {
            return Response::new(
                StatusCode::BAD_REQUEST,
                Some(ErrorResponseBody {
                    error: "Invalid username or password".to_string(),
                }.to_bytes()),
            );
        }

        Response::new(
            StatusCode::OK,
            Some(LoginResponseBody {
                token: TEMPORARY_JWT_TOKEN.to_string(), // todo create a real jwt token
            }.to_bytes()),
        )
    }

    pub fn handle_register(&self, ctx: &dyn ContextTrait) -> Response {
        let data = ctx.request_body();
        let (body, error, status) = Self::parse_request_body::<RegisterRequestBody>(data);
        if status != StatusCode::OK {
            return Response::new(
                status,
                error.map(|e| e.to_bytes()),
            );
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        let mut users = USERS.lock().unwrap(); // Lock the USERS vector to ensure thread safety

        // Check if the username already exists
        if users.iter().any(|(user, _)| user == &request_body.username) {
            let response_body = ErrorResponseBody {
                error: "Username already exists".to_string(),
            };
            return Response::new(
                StatusCode::BAD_REQUEST,
                Some(response_body.to_bytes()),
            );
        }

        // Save new user in memory
        users.push((request_body.username, request_body.password));

        Response::new(
            StatusCode::OK,
            Some(RegisterResponseBody {
                success: true,
                message: "User registered successfully".to_string(),
            }.to_bytes()),
        )
    }

    pub fn authentication_middleware(&self, ctx: &dyn ContextTrait) -> Response {
        let token = ctx.request_header().headers.get("Authorization").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
        if token.is_none() || token.unwrap() != TEMPORARY_JWT_TOKEN {
            return Response::new(
                StatusCode::UNAUTHORIZED,
                Some(ErrorResponseBody {
                    error: "Unauthorized".to_string(),
                }.to_bytes()),
            );
        }
        Response::new(StatusCode::OK, None)
    }

    pub fn get_profile(&self, ctx: &dyn ContextTrait) -> Response {
        let response_body = GetProfileResponse { // todo fetch from database
            name: "ChatGPT".to_string(),
            title: "AI Assistant by OpenAI".to_string(),
            avatar: "https://upload.wikimedia.org/wikipedia/commons/0/04/ChatGPT_logo.svg".to_string(),
            bio: "ChatGPT is a language model designed to assist with writing, coding, learning, and more. \
            Trained on a wide range of data, it aims to provide accurate, clear, and human-like responses to support users in diverse tasks.".to_string(),
            email: "Not applicable 😊".to_string(),
            location: "The Cloud ☁️".to_string(),
            website: "https://openai.com/chatgpt".to_string(),
        };
        Response::new(StatusCode::OK, Some(response_body.to_bytes()))
    }
}