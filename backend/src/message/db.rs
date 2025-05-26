use std::collections::HashMap;
use crate::message::types::other::{Poem, UserMetadata};

pub struct WGPDatabase {
    user_password: HashMap<String, String>,
    user_metadata: HashMap<String, UserMetadata>,
    poems: Box<[Poem]>
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

            poems: Box::new([
                Poem {
                    id: 1,
                    title: "The Road Not Taken".to_string(),
                    author: "Robert Frost".to_string(),
                    content: "Two roads diverged in a yellow wood,\nAnd sorry I could not travel both...".to_string(),
                },
                Poem {
                    id: 2,
                    title: "Still I Rise".to_string(),
                    author: "Maya Angelou".to_string(),
                    content: "You may write me down in history\nWith your bitter, twisted lies...".to_string(),
                },
                Poem {
                    id: 3,
                    title: "Ozymandias".to_string(),
                    author: "Percy Bysshe Shelley".to_string(),
                    content: "I met a traveller from an antique land\nWho said—“Two vast and trunkless legs of stone...".to_string(),
                },
                Poem {
                    id: 4,
                    title: "If—".to_string(),
                    author: "Rudyard Kipling".to_string(),
                    content: "If you can keep your head when all about you\nAre losing theirs and blaming it on you...".to_string(),
                },
                Poem {
                    id: 5,
                    title: "Annabel Lee".to_string(),
                    author: "Edgar Allan Poe".to_string(),
                    content: "It was many and many a year ago,\nIn a kingdom by the sea...".to_string(),
                }
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

    pub fn get_poems(&self) -> &[Poem] {
        &self.poems
    }

    pub fn get_poem(&self, id: &str) -> Option<&Poem> {
        self.poems.iter().find(|&poem| poem.id.to_string() == id)
    }
}