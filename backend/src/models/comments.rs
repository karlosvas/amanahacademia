use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub uid: String,
    pub name: String,
    pub fecha: String,
    pub content: String,
    pub url_img: String,
    pub like: u32,
    #[serde(default)]
    pub reply: Vec<Comment>,
    #[serde(default)]
    pub users_liked: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRequest {
    pub name: String,
    pub fecha: String,
    pub content: String,
    pub url_img: String,
}

#[derive(Deserialize)]
pub struct FirebaseCommentResponse {
    pub name: String,
}
