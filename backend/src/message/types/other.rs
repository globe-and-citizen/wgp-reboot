use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMetadata {
    pub username: String,
    pub title: String,
    pub avatar: String,
    pub bio: String,
    pub email: String,
    pub location: String,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poem {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
}