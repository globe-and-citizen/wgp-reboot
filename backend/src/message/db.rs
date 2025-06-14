use std::collections::HashMap;
use std::fs;
use std::{path::PathBuf};
use crate::message::ntor::server::{Server as nTorServer};
use crate::message::types::other::{Image, Poem, UserMetadata};

pub struct WGPDatabase {
    user_password: HashMap<String, String>,
    user_metadata: HashMap<String, UserMetadata>,
    poems: Box<[Poem]>,
    images: Box<[Image]>,
    ntor_sessions: HashMap<String, nTorServer>
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

            images: Box::new([
                Image {
                    id: 1,
                    name: "Sample Image 1".to_string(),
                    file_path: "src/message/images".to_string(),
                    file_name: "sample1.jpeg".to_string(),
                    content: vec![],
                },
                Image {
                    id: 2,
                    name: "Sample Image 2".to_string(),
                    file_path: "src/message/images".to_string(),
                    file_name: "sample2.jpeg".to_string(),
                    content: vec![],
                },
                Image {
                    id: 3,
                    name: "Sample Image 3".to_string(),
                    file_path: "src/message/images".to_string(),
                    file_name: "sample3.jpeg".to_string(),
                    content: vec![],
                },
                Image {
                    id: 4,
                    name: "Sample Image 4".to_string(),
                    file_path: "src/message/images".to_string(),
                    file_name: "sample4.jpeg".to_string(),
                    content: vec![],
                },
                Image {
                    id: 5,
                    name: "Sample Image 5".to_string(),
                    file_path: "src/message/images".to_string(),
                    file_name: "sample5.jpeg".to_string(),
                    content: vec![],
                },
            ]),

            ntor_sessions: HashMap::new()
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

    pub fn get_image(&mut self, id: &str) -> Result<&Image, String> {
        for wgp_img in self.images.iter_mut() {
            if wgp_img.id.to_string() == id {
                if !wgp_img.content.is_empty() {
                    return Ok(wgp_img)
                }

                let abs_path = format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), wgp_img.file_path, wgp_img.file_name);
                let mut path = PathBuf::from("images");
                path.push(&abs_path);

                return match fs::read(&path) {
                    Ok(image_data) => {
                        wgp_img.content = image_data.into();
                        println!("Read {} bytes from {}", wgp_img.content.len(), abs_path);
                        Ok(wgp_img)
                    }
                    Err(err) => {
                        Err(format!("Failed to open image file {}: {}", wgp_img.file_path, err))
                    }
                }
            };
        }
        Err(format!("Image with id {} not found", id))
    }

    pub fn get_images(&self) -> &Box<[Image]> {
        &self.images
    }

    pub fn save_ntor_session(&mut self, session_id: &str, server: nTorServer) -> Option<nTorServer> {
        self.ntor_sessions.insert(session_id.to_string(), server)
    }
    pub fn get_ntor_session(&self, session_id: &str) -> Option<&nTorServer> {
        self.ntor_sessions.get(session_id)
    }
}