use log::debug;
use crate::message::entities::{CustomRequestBody, CustomResponseBody, RequestBody, ResponseBody};

pub struct MessageHandler {
    // whatever can be added later as needed
}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {}
    }

    pub fn handle_example(&self, data: &Vec<u8>, ) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {

        let request_body = CustomRequestBody::from_bytes(data.clone());
        // let request_body: CustomRequestBody = serde_json::de::from_slice::<CustomRequestBody>(&request_body).unwrap();
        debug!("Request body: {:?}", request_body);

        Ok(Some((CustomResponseBody {
            result: "ok".to_string(),
        }).to_bytes()))
    }
}