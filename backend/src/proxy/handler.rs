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

    fn extract_request_summary(session: &Session) -> (String, String) {
        let request_summary = session.request_summary();
        let parts: Vec<&str> = request_summary.split_whitespace().collect();

        if parts.len() > 1 {
            let method = parts[0].to_string();
            let path = parts[1]
                .trim_end_matches(',')
                // .split('/')
                // .collect::<Vec<&str>>()
                // .get(1) // todo
                // .unwrap_or(&"")
                .to_string();
            (method, path)
        } else {
            error!("Invalid request summary: {}", request_summary);
            (String::new(), String::new())
        }
    }

    pub(crate) fn validate_request(&self, session: &Session) -> StatusCode {
        let (method, path) = ProxyHandler::<T>::extract_request_summary(session);

        println!("method: {}, path: {}", method, path);
        if self.router.contains(&Method::POST, &path) {
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    }

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

    pub(crate) async fn handle_request(&self, session: &mut Session) -> pingora::Result<Vec<u8>> {
        // read request body
        match ProxyHandler::<T>::get_request_body(session).await {
            Ok(request_body) => {

                let (_, path) = ProxyHandler::<T>::extract_request_summary(session);

                let response = self.router.call_handler(&Method::POST, &path, &request_body);

                match response {
                    Ok(Some(res)) => {
                        debug!("Response body: {:?}", res);
                        Ok(res)
                    }
                    Ok(None) => {
                        error!("No handler found for {}: {}", Method::POST, path);
                        Ok(vec![])
                    }
                    Err(err) => {
                        error!("ERROR: {err}");
                        Err(pingora::Error::because(pingora::ErrorType::InternalError, "Error in handler", err))
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