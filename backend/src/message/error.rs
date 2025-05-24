use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum MessageHandlerError {
    InternalError(Box<dyn std::error::Error + Send + Sync>),
    ParseBodyError(Box<dyn std::error::Error + Send + Sync>),
    InvalidData(&'static str),
}

impl MessageHandlerError {
    pub fn to_pingora_error(&self) -> pingora::BError {
        match self {
            MessageHandlerError::InternalError(err) =>
                pingora::Error::because(pingora::ErrorType::InternalError, "Error from MessageHandler", format!("{:?}", err)),
            MessageHandlerError::ParseBodyError(err) =>
                pingora::Error::because(pingora::ErrorType::BindError, "Error from MessageHandler", format!("{:?}", err)),
            MessageHandlerError::InvalidData(msg) =>
                pingora::Error::explain(pingora::ErrorType::Custom(msg), "Error from MessageHandler"),
        }
    }
}

impl fmt::Display for MessageHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageHandlerError::InternalError(err) => write!(f, "Internal server error: {:?}", err),
            MessageHandlerError::ParseBodyError(err) => write!(f, "Failed to parse request body: {:?}", err),
            MessageHandlerError::InvalidData(msg) => write!(f, "Invalid request data: {}", msg),
        }
    }
}

impl Error for MessageHandlerError {}

impl From<std::io::Error> for MessageHandlerError {
    fn from(err: std::io::Error) -> MessageHandlerError {
        MessageHandlerError::InternalError(err.into())
    }
}

