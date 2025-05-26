use std::collections::HashMap;
use crate::message::types::other::UserMetadata;

pub struct WGPDatabase {
    user_password: HashMap<String, String>,
    user_metadata: HashMap<String, UserMetadata>,
}

impl WGPDatabase {
    pub fn new() -> Self {
        WGPDatabase {
            user_password: HashMap::from([
                ("tester".to_string(), "1234".to_string()),
            ]),

            user_metadata: HashMap::from([
                ("tester".to_string(), UserMetadata {
                    username: "tester".to_string(),
                    title: "AI Assistant by OpenAI".to_string(),
                    avatar: "https://upload.wikimedia.org/wikipedia/commons/0/04/ChatGPT_logo.svg".to_string(),
                    bio: "ChatGPT is a language model designed to assist with writing, coding, learning, and more. \
            Trained on a wide range of data, it aims to provide accurate, clear, and human-like responses to support users in diverse tasks.".to_string(),
                    email: "Not applicable 😊".to_string(),
                    location: "The Cloud ☁️".to_string(),
                    website: "https://openai.com/chatgpt".to_string(),
                })
            ]),
        }
    }

    pub fn add_user(&mut self, username: String, password: String, metadata: UserMetadata) {
        self.user_password.insert(username.clone(), password);
        self.user_metadata.insert(username, metadata);
    }

    pub fn get_password(&self, username: &str) -> Option<&String> {
        self.user_password.get(username)
    }

    pub fn get_metadata(&self, username: &str) -> Option<&UserMetadata> {
        self.user_metadata.get(username)
    }

    pub fn user_exists(&self, username: &str) -> bool {
        self.user_password.contains_key(username)
    }
}