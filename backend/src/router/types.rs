use pingora::http::{Method, RequestHeader, StatusCode};
use pingora::prelude::Session;

pub trait ContextTrait {
    fn method(&self) -> Method;
    fn path(&self) -> &str;
    fn request_header(&self) -> &RequestHeader;
    fn request_body(&self) -> &Vec<u8>;
    fn session(&self) -> &Session;

    /// get and set simple key-value pairs to use throughout the request handling
    fn get(&self, key: &str) -> Option<&String>;
    fn set(&mut self, key: String, value: String);
}

pub struct Response {
    pub status: StatusCode,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<Vec<u8>>) -> Self {
        Response { status, body }
    }
}

// Box<dyn std::error::Error + Send + Sync> is used to represent any error type that implements the std::error::Error trait and can be sent across thread boundaries.
pub type HandleMessage<T> = fn(&T, &mut dyn ContextTrait) -> Response;