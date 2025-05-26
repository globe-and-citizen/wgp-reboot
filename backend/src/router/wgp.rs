use pingora::proxy::Session;
use pingora::http::{Method, RequestHeader};
use crate::router::types::ContextTrait;

/// WGPContext is a context for handling WGP requests.
/// It implements the ContextTrait to provide access to request details and session information.
/// It is expected to simplify or customize the usage of pingora::proxy::Session, particularly for this repository.
pub struct WGPContext<'a> {
    method: Method,
    path: String,
    request_header: &'a RequestHeader,
    request_body: Vec<u8>,
    session: &'a Session,
}

impl<'a> WGPContext<'a> {
    pub(crate) fn new(method: Method, path: String, body: Vec<u8>, session: &'a Session) -> Self {
        WGPContext {
            method,
            path,
            request_header: session.req_header(),
            request_body: body,
            session,
        }
    }
}

impl<'a> ContextTrait for WGPContext<'a> {
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
}