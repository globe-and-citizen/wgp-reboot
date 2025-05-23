use std::panic::panic_any;
use log::{debug, error};
use pingora::http::{Method, ResponseHeader, StatusCode};
use pingora::prelude::Session;
use crate::entities::{CustomRequestBody, CustomResponseBody, ResponseBody};


pub(crate) struct ProxyHandler {
    routes: Vec<String>,
}

impl ProxyHandler {
    pub(crate) fn new() -> Self {

        // todo routes are hardcoded for now
        let routes = vec![
            "api_path".to_string(),
        ];

        ProxyHandler { routes }
    }

    fn extract_request_summary(session: &Session) -> (String, String) {
        let request_summary = session.request_summary();
        let parts: Vec<&str> = request_summary.split_whitespace().collect();

        if parts.len() > 1 {
            let method = parts[0].to_string();
            let path = parts[1]
                .split('/')
                .collect::<Vec<&str>>()
                .get(1) // todo
                .unwrap_or(&"")
                .trim_end_matches(',')
                .to_string();
            (method, path)
        } else {
            error!("Invalid request summary: {}", request_summary);
            (String::new(), String::new())
        }
    }

    pub(crate) fn validate_request(&self, session: &Session) -> StatusCode {
        let (method, path) = ProxyHandler::extract_request_summary(session);

        // only POST method is allowed for now
        if method == Method::POST.to_string() {
            // check if path is allowed
            if self.routes.contains(&path) {
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
            // browser always sends an OPTIONS request along with POST for 'application/json' content-type
        } else if method == Method::OPTIONS.to_string() {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::METHOD_NOT_ALLOWED
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

    pub(crate) async fn handle_request(&self, session: &mut Session) -> pingora::Result<Option<impl ResponseBody>> {
        // read request body
        match ProxyHandler::get_request_body(session).await {
            Ok(request_body) => {

                let (_, route) = ProxyHandler::extract_request_summary(session);
                match route.as_str() {
                    "api_path" => {
                        // todo
                        // convert to json
                        let request_body: CustomRequestBody = serde_json::de::from_slice::<CustomRequestBody>(&request_body).unwrap();
                        debug!("Request body: {:?}", request_body);

                        Ok(Some(CustomResponseBody {
                            result: "ok".to_string(),
                        }))
                    }
                    _ => {
                        panic_any("this line shouldn't be reached because of the validate_request method");
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