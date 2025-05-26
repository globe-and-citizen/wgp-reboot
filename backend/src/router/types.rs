use pingora::http::{Method, RequestHeader, StatusCode};
use pingora::proxy::Session;

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

/// Context is a context for handling WGP requests.
/// It implements the ContextTrait to provide access to request details and session information.
/// It is expected to simplify or customize the usage of pingora::proxy::Session, particularly for this repository.
pub struct Context<'a> {
    method: Method,
    path: String,
    request_header: &'a RequestHeader,
    request_body: Vec<u8>,
    session: &'a Session,
    memory: std::collections::HashMap<String, String>, // for storing key-value pairs
}

impl<'a> Context<'a> {
    pub(crate) fn new(method: Method, path: String, body: Vec<u8>, session: &'a Session) -> Self {
        Context {
            method,
            path,
            request_header: session.req_header(),
            request_body: body,
            session,
            memory: std::collections::HashMap::new(), // Initialize an empty HashMap for memory
        }
    }
}

impl<'a> ContextTrait for Context<'a> {
    fn method(&self) -> Method {
        self.method.clone()
    }

    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn request_header(&self) -> &RequestHeader {
        self.request_header
    }

    fn request_body(&self) -> &Vec<u8> {
        &self.request_body
    }

    fn session(&self) -> &Session {
        self.session
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.memory.get(key)
    }

    fn set(&mut self, key: String, value: String) {
        self.memory.insert(key, value);
    }
}