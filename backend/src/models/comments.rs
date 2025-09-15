use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_uid: Option<String>, // Usuario que comento
    pub name: String,
    pub timestamp: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_img: Option<String>,
    pub stars: f32,
    #[serde(default)]
    pub like: u32,
    #[serde(default)]
    pub reply: Vec<Comment>,
    #[serde(default)]
    pub users_liked: Vec<String>,
}
