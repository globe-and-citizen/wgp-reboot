use std::string::ToString;
use std::sync::{Arc, Mutex, MutexGuard};
use log::{debug, error};
use pingora::http::StatusCode;
use crate::config::HandlerConfig;
use crate::message::types::{RequestBodyTrait, ResponseBodyTrait, NTorEncryptMessage};
use crate::message::types::request::{LoginRequestBody, RegisterRequestBody, NTorInitRequestBody};
use crate::message::types::response::{ErrorResponseBody, GetProfileResponse, LoginResponseBody, RegisterResponseBody, GetPoemsResponse, GetPoemResponse, GetImageResponse, GetImagesResponse, NTorInitResponse};
use crate::message::types::other::{UserMetadata};
use crate::message::db::WGPDatabase;
use crate::message::ntor::server::{Server as nTorServer};
use crate::message::ntor::common::{InitSessionMessage};
use crate::message::utils::{create_jwt_token, new_nTor_session_id, string_to_array32, verify_jwt_token};
use crate::router::types::{ContextTrait, Response};

pub struct WGPMessageHandler {
    config: HandlerConfig,
    ntor_static_secret: [u8; 32],
    jwt_secret: [u8; 32],
    // use std::sync::Mutex to make db mutable without requiring WGPMessageHandler itself to be mutable,
    // and use an Arc if we need shared ownership across threads.
    db: Arc<Mutex<WGPDatabase>>,
}

impl WGPMessageHandler {
    pub fn new(config: HandlerConfig) -> Self {
        let ntor_secret = string_to_array32(config.ntor_static_secret.clone()).unwrap();
        let jwt_secret = string_to_array32(config.jwt_secret.clone()).unwrap();

        WGPMessageHandler {
            config,
            ntor_static_secret: ntor_secret,
            jwt_secret,
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
        let data = ctx.get_request_body();
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
                        token: create_jwt_token(request_body.username, self.jwt_secret),
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
        let data = ctx.get_request_body();
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
        let token = ctx.request_header().headers.get("Authorization")
            .and_then(|v| v.to_str().ok()).map(|s| s.to_string());

        if token.is_none() {
            return Response::new(
                StatusCode::UNAUTHORIZED,
                Some(ErrorResponseBody {
                    error: "Unauthorized".to_string(),
                }.to_bytes()),
            );
        }

        let token = token.unwrap().replace(&"Bearer ".to_string(), &"".to_string());
        debug!("token {}", token);

        return match verify_jwt_token(&token, self.jwt_secret) {
            Ok(token_claim) => {
                let claims = token_claim.claims;

                // get login credentials from the authorization token
                let username = claims.get_username();
                debug!("username: {}", username);
                if username.is_empty() {
                    return Response::new(
                        StatusCode::UNAUTHORIZED,
                        Some(ErrorResponseBody {
                            error: "Invalid token".to_string(),
                        }.to_bytes()),
                    );
                }

                let db = self.get_db();
                if !db.user_exists(&username) {
                    return Response::new(
                        StatusCode::UNAUTHORIZED,
                        Some(ErrorResponseBody {
                            error: "User does not exist".to_string(),
                        }.to_bytes()),
                    );
                }

                // set credentials in the context for further use
                ctx.set("username".to_string(), username);

                // If the token is valid, continue processing the request
                Response::new(StatusCode::OK, None)
            }
            Err(err) => {
                error!("Validate token error {err:?}");
                Response::new(
                    StatusCode::UNAUTHORIZED,
                    Some(ErrorResponseBody {
                        error: "Invalid token".to_string(),
                    }.to_bytes()),
                )
            }
        };
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

    pub fn get_images(&self, ctx: &mut dyn ContextTrait) -> Response {
        let mut db = self.get_db();
        let id = ctx.param("id");

        // If an id is provided, fetch the specific image
        if let Some(id) = id {
            return match db.get_image(&id) {
                Ok(image) => {
                    let response_body = GetImageResponse {
                        id: image.id,
                        title: image.name.clone(),
                        file_name: image.file_name.clone(),
                        content: image.content.clone(),
                    };

                    Response::new(StatusCode::OK, Some(response_body.to_bytes()))
                }
                Err(err) => {
                    error!("ERROR: {}", err);
                    Response::new(
                        StatusCode::NOT_FOUND,
                        Some(ErrorResponseBody {
                            error: err,
                        }.to_bytes()),
                    )
                }
            };
        }

        // If no id is provided, return all images
        let images = db.get_images();
        let img_response = images.into_iter().map(|img| GetImageResponse {
            id: img.id,
            title: img.name.clone(),
            file_name: img.file_name.clone(),
            content: img.content.clone(),
        }).collect::<Vec<GetImageResponse>>();

        let response_body = GetImagesResponse {
            images: Box::from(img_response)
        };
        Response::new(StatusCode::OK, Some(response_body.to_bytes()))
    }

    pub fn ntor_init(&self, ctx: &mut dyn ContextTrait) -> Response {
        let data = ctx.get_request_body();
        let (body, error, status) = Self::parse_request_body::<NTorInitRequestBody>(data);
        if status != StatusCode::OK {
            return Response::new(status, error.map(|e| e.to_bytes()));
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        // todo I think there are prettier ways to use nTor since we are free to modify the nTor crate, but I'm lazy
        let mut ntor_server = nTorServer::new_with_secret(
            self.config.ntor_server_id.clone(),
            self.ntor_static_secret,
        );

        if request_body.public_key.len() != 32 {
            return Response::new(StatusCode::BAD_REQUEST, None);
        }

        // Client initializes session with the server
        let init_session_msg = InitSessionMessage::from(request_body.public_key);

        let init_session_response = ntor_server.accept_init_session_request(&init_session_msg);

        let ntor_session_id = new_nTor_session_id();

        let response = NTorInitResponse {
            public_key: Vec::from(init_session_response.server_ephemeral_public_key.to_bytes()),
            t_hash: init_session_response.t_hash,
            session_id: ntor_session_id.clone(),
            static_public_key: ntor_server.get_certificate().public_key.to_bytes().to_vec(),
            server_id: self.config.ntor_server_id.clone()
        };

        // save nTor session
        let mut db = self.get_db();
        db.save_ntor_session(&ntor_session_id, ntor_server);

        Response::new(
            StatusCode::OK,
            Some(response.to_bytes()),
        )
    }

    pub fn ntor_encrypt(&self, ctx: &mut dyn ContextTrait) -> Response {
        let response_bytes = ctx.get_response_body();
        let db = self.get_db();

        // todo consider where to put ntor session id
        let ntor_session_id = ctx.request_header().headers.get("nTor_session_id")
            .and_then(|v| v.to_str().ok()).map(|s| s.to_string());

        if let Some(session_id) = ntor_session_id {
            debug!("Session id: {}", session_id);
            let ntor_server = db.get_ntor_session(&session_id);
            // debug!("Response bytes: {}", hex::encode(response_bytes.clone()));

            if let Some(server) = ntor_server {
                return match server.encrypt(response_bytes.clone()) {
                    Ok((nonce, encrypted)) => {
                        // let decrypted = server.decrypt(nonce.clone(), encrypted.clone()).unwrap();
                        // println!("Decrypted bytes: {}", hex::encode(decrypted));
                        Response::new(
                            StatusCode::OK,
                            Some(NTorEncryptMessage { nonce, encrypted }.to_bytes()),
                        )
                    }
                    Err(err) => {
                        error!("unable to encrypt: {}", err);
                        Response::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            None,
                        )
                    }
                }
            }
        }

        Response::new(
            StatusCode::BAD_REQUEST,
            Some(ErrorResponseBody { error: "no nTor session found".to_string() }.to_bytes()),
        )
    }

    pub fn ntor_decrypt(&self, ctx: &mut dyn ContextTrait) -> Response {
        let data = ctx.get_request_body();
        let (body, error, status) = Self::parse_request_body::<NTorEncryptMessage>(data);
        if status != StatusCode::OK {
            return Response::new(status, error.map(|e| e.to_bytes()));
        }

        let request_body = body.unwrap(); // Unwrap the Option, safe because we checked status

        // todo consider where to put ntor session id
        let ntor_session_id = ctx.request_header().headers.get("nTor_session_id")
            .and_then(|v| v.to_str().ok()).map(|s| s.to_string());

        if let Some(session_id) = ntor_session_id {
            debug!("Session id: {}", session_id);
            let db = self.get_db();
            let ntor_server = db.get_ntor_session(&session_id);

            if let Some(server) = ntor_server {
                return match server.decrypt(request_body.nonce, request_body.encrypted) {
                    Ok(decrypted) => {
                        ctx.set_request_body(decrypted);
                        Response::new(
                            StatusCode::OK,
                            None,
                        )
                    }
                    Err(err) => {
                        error!("unable to decrypt: {}", err);
                        Response::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            None,
                        )
                    }
                }
            }
        }

        Response::new(
            StatusCode::BAD_REQUEST,
            Some(ErrorResponseBody { error: "no nTor session found".to_string() }.to_bytes()),
        )
    }

}