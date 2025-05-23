use log::{debug, error};
use pingora::http::{Method, ResponseHeader, StatusCode};
use pingora::prelude::Session;
use crate::router::Router;

pub(crate) struct ProxyHandler<T> {
    router: Router<T>,
}

impl<T> ProxyHandler<T> {
    pub(crate) fn new(router: Router<T>) -> Self {
        ProxyHandler { router }
    }

    /// Extracts the request method and path from the session.
    /// # Arguments
    /// * `session` - A reference to the session object containing the request summary.
    /// # Returns
    /// * A tuple containing the HTTP method and the path as strings.
    /// # Errors
    /// * Returns an error if the method is unsupported or if the request summary is invalid (if it doesn't contain at least two parts (method and path)).
    fn extract_request_summary(session: &Session) -> Result<(Method, String), String> {
        let request_summary = session.request_summary();
        let parts: Vec<&str> = request_summary.split_whitespace().collect();

        if parts.len() > 1 {
            let method = match parts[0].to_string().as_str() {
                "POST" => Method::POST,
                "GET" => Method::GET,
                "PUT" => Method::PUT,
                "DELETE" => Method::DELETE,
                _ => {
                    error!("Unsupported method: {}", parts[0].to_string());
                    return Err("Unsupported method")?;
                }
            };

            let path = parts[1]
                .trim_end_matches(',')
                // .split('/')
                // .collect::<Vec<&str>>()
                // .get(1) // todo
                // .unwrap_or(&"")
                .to_string();

            Ok((method, path))
        } else {
            Err("Invalid request summary".to_string())
        }
    }

    /// Validates the request by checking if the method and path are supported.
    /// # Arguments
    /// * `session` - A reference to the session object containing the request summary.
    /// # Returns
    /// * A StatusCode indicating the result of the validation.
    pub(crate) fn validate_request(&self, session: &Session) -> StatusCode {
        let (method, path) = match ProxyHandler::<T>::extract_request_summary(session) {
            Ok((method, path)) => (method, path),
            Err(err) => {
                error!("ERROR: {err}");
                return StatusCode::METHOD_NOT_ALLOWED;
            }
        };

        if self.router.contains(&method, &path) {
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    }

    /// Reads the request body from the session.
    /// # Arguments
    /// * `session` - A mutable reference to the session object.
    /// # Returns
    /// * A Result containing the request body as a Vec<u8> or an error.
    /// # Errors
    /// * Returns an error if reading the request body fails.
    async fn get_request_body(session: &mut Session) -> pingora::Result<Vec<u8>> {
        // read request body
        let mut body = Vec::new();
        loop {
            match session.read_request_body().await {
                Ok(option) => {
                    match option {
                        Some(chunk) => body.extend_from_slice(&chunk),
                        None => break,
                    }
                }
                Err(err) => {
                    error!("ERROR: {err}");
                    return Err(err);
                }
            }
        }
        Ok(body)
    }

    /// Handles the request by extracting the method and path, and calling the appropriate handler.
    /// # Arguments
    /// * `session` - A mutable reference to the session object.
    /// # Returns
    /// * A Result containing the response body as a Vec<u8> or an error.
    /// # Errors
    /// * Returns an error if the request body cannot be read or if the handler fails.
    pub(crate) async fn handle_request(&self, session: &mut Session) -> pingora::Result<Vec<u8>> {
        // read request body
        match ProxyHandler::<T>::get_request_body(session).await {
            Ok(request_body) => {

                // request_validation is called before this function, so we can assume that the request is valid
                let (method, path) = ProxyHandler::<T>::extract_request_summary(session).unwrap();

                let response = self.router.call_handler(&method, &path, &request_body);

                match response {
                    Ok(Some(res)) => {
                        debug!("Response body: {:?}", res);
                        Ok(res)
                    }
                    Ok(None) => {
                        error!("No handler found for {}: {}", method, path); // todo check if this is ok
                        Ok(vec![])
                    }
                    Err(err) => {
                        error!("ERROR: {err}");
                        Err(pingora::Error::because(pingora::ErrorType::InternalError, "Error in handler", err)) // todo check for request body error
                    }
                }
            }
            Err(err) => {
                error!("ERROR: {err}");
                Err(err)
            }
        }
    }

    pub(crate) async fn set_headers(response_status: StatusCode, body_bytes: &Vec<u8>, session: &mut Session) -> pingora::Result<()> {
        let mut header = ResponseHeader::build(response_status, None)?;
        header.append_header("Content-Length", body_bytes.len().to_string()).unwrap();
        // access headers below are needed to pass browser's policy
        header.append_header("Access-Control-Allow-Origin", "*".to_string()).unwrap();
        header.append_header("Access-Control-Allow-Methods", "POST".to_string()).unwrap();
        header.append_header("Access-Control-Allow-Headers", "Content-Type".to_string()).unwrap();
        header.append_header("Access-Control-Max-Age", "86400".to_string()).unwrap();
        session.write_response_header_ref(&header).await
    }
}