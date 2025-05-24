use log::debug;
use crate::message::entities::{LoginRequestBody, LoginResponseBody, RequestBody, ResponseBody};
use crate::message::error::MessageHandlerError;

pub struct MessageHandler {
    // whatever can be added later as needed
}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {}
    }

    pub fn handle_login(&self, data: &Vec<u8>, ) -> Result<Option<Vec<u8>>, pingora::BError> {

        let request_body = match LoginRequestBody::from_bytes(data.clone()) {
            Ok(body) => body,
            Err(e) => {
                debug!("Failed to parse request body: {:?}", e);
                return Err(MessageHandlerError::ParseBodyError(e.into()).to_pingora_error());
            }
        };
        debug!("Request body: {:?}", request_body);

        if request_body.username != "tester" || request_body.password != "1234" {
            return Err(MessageHandlerError::InvalidData("Invalid username or password").to_pingora_error());
        }

        Ok(Some((LoginResponseBody {
            token: "jwt_token".to_string(), //todo create a real jwt token
        }).to_bytes()))
    }
}