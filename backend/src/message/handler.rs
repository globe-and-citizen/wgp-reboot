use std::string::ToString;
use std::sync::{Arc, Mutex, MutexGuard};
use log::error;
use pingora::http::StatusCode;
use crate::message::types::request::{RequestBodyTrait, LoginRequestBody, RegisterRequestBody};
use crate::message::types::response::{ResponseBodyTrait, ErrorResponseBody, GetProfileResponse, LoginResponseBody, RegisterResponseBody, GetPoemsResponse, GetPoemResponse};
use crate::message::types::other::UserMetadata;
use crate::message::db::WGPDatabase;
use crate::message::utils::{create_jwt_token, get_username_from_token};
use crate::router::types::{ContextTrait, Response};

pub struct WGPMessageHandler {
    // use std::sync::Mutex to make db mutable without requiring WGPMessageHandler itself to be mutable,
    // and use an Arc if we need shared ownership across threads.
    db: Arc<Mutex<WGPDatabase>>,
}

impl WGPMessageHandler {
    pub fn new() -> Self {
        WGPMessageHandler {
            db: Arc::new(Mutex::new(WGPDatabase::new())),
        }
    }

    fn get_db(&self) -> MutexGuard<'_, WGPDatabase> {
        self.db.lock().unwrap()
    }

    fn parse_request_body<T: RequestBodyTrait>(data: &Vec<u8>) -> (Option<Box<T>>, Option<ErrorResponseBody>, StatusCode) {
        match T::from_bytes(data.clone()) {
            Ok(body) => (Some(body), None, StatusCode::OK),
            Err(e) => {
                (None, Some(ErrorResponseBody {
                    error: e.to_string(),
                }), StatusCode::BAD_REQUEST)
            }
        }
    }

    pub fn handle_login(&self, ctx: &mut dyn ContextTrait) -> Response {
        let data = ctx.request_body();
        let (body, error, status) = Self::parse_request_body::<LoginRequestBody>(data);
        if status != StatusCode::OK {
            return Response::new(status, error.map(|e| e.to_bytes()));
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        if let Some(password) = self.get_db().get_password(&request_body.username) {
            if *password == request_body.password {
                return Response::new(
                    StatusCode::OK,
                    Some(LoginResponseBody {
                        token: create_jwt_token(request_body.username),
                    }.to_bytes()),
                );
            }
        }

        Response::new(
            StatusCode::BAD_REQUEST,
            Some(ErrorResponseBody {
                error: "Invalid username or password".to_string(),
            }.to_bytes()),
        )
    }

    pub fn handle_register(&self, ctx: &mut dyn ContextTrait) -> Response {
        let data = ctx.request_body();
        let (body, error, status) = Self::parse_request_body::<RegisterRequestBody>(data);
        if status != StatusCode::OK {
            return Response::new(
                status,
                error.map(|e| e.to_bytes()),
            );
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        let mut db = self.get_db();
        if db.user_exists(&request_body.username) {
            return Response::new(
                StatusCode::BAD_REQUEST,
                Some(ErrorResponseBody {
                    error: "Username already exists".to_string(),
                }.to_bytes()),
            );
        }

        db.add_user(request_body.username.clone(), request_body.password.clone(), UserMetadata {
            username: request_body.username.clone(),
            title: "".to_string(),
            avatar: "".to_string(),
            bio: "".to_string(),
            email: "".to_string(),
            location: "".to_string(),
            website: "".to_string(),
        });

        Response::new(
            StatusCode::OK,
            Some(RegisterResponseBody {
                success: true,
                message: "User registered successfully".to_string(),
            }.to_bytes()),
        )
    }

    pub fn authentication_middleware(&self, ctx: &mut dyn ContextTrait) -> Response {
        let token = ctx.request_header().headers.get("Authorization").and_then(|v| v.to_str().ok()).map(|s| s.to_string());

        if token.is_none() { // todo update authorization logic
            return Response::new(
                StatusCode::UNAUTHORIZED,
                Some(ErrorResponseBody {
                    error: "Unauthorized".to_string(),
                }.to_bytes()),
            );
        }
        let token = token.unwrap();

        // get login credentials from the authorization token
        let username = get_username_from_token(&token);
        if username.is_none() {
            return Response::new(
                StatusCode::UNAUTHORIZED,
                Some(ErrorResponseBody {
                    error: "Invalid token".to_string(),
                }.to_bytes()),
            );
        }

        let db = self.get_db();
        if !db.user_exists(&username.unwrap()) {
            return Response::new(
                StatusCode::UNAUTHORIZED,
                Some(ErrorResponseBody {
                    error: "User does not exist".to_string(),
                }.to_bytes()),
            );
        }

        // set credentials in the context for further use
        ctx.set("username".to_string(), username.unwrap().to_string());

        // If the token is valid, continue processing the request
        Response::new(StatusCode::OK, None)
    }

    pub fn get_profile(&self, ctx: &mut dyn ContextTrait) -> Response {
        if let Some(username) = ctx.get("username") {
            let db = self.get_db();
            let metadata = db.get_metadata(username);

            let response_body = GetProfileResponse {
                metadata: metadata.unwrap().clone()
            };

            Response::new(StatusCode::OK, Some(response_body.to_bytes()))
        } else {
            error!("ERROR: Username not found in context");
            Response::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            )
        }
    }

    pub fn get_poems(&self, ctx: &mut dyn ContextTrait) -> Response {
        let db = self.get_db();
        let id = ctx.param("id");

        if let Some(id) = id {
            if let Some(poem) = db.get_poem(&id) {
                let response_body = GetPoemResponse {
                    id: poem.id,
                    title: poem.title.to_string(),
                    author: poem.author.to_string(),
                    content: poem.content.to_string(),
                };

                return Response {
                    status: StatusCode::OK,
                    body: Some(response_body.to_bytes()),
                };
            }
            return Response {
                status: StatusCode::NOT_FOUND,
                body: Some(ErrorResponseBody {
                    error: "Id not found".to_string(),
                }.to_bytes()),
            };
        }


        let response_body = GetPoemsResponse {
            poems: Box::from(db.get_poems())
        };

        Response::new(StatusCode::OK, Some(response_body.to_bytes()))
    }
}