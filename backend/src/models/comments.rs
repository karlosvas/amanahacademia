use serde::{Deserialize, Serialize};

/// Comentario en Firebase DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub author_uid: Option<String>, // Usuario que comento
    pub name: String,               // Nombre del usuario
    pub timestamp: String,          // Fecha del comentario
    pub content: String,            // Contenido del comentario
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_img: Option<String>, // Imagen del usuario
    pub stars: f32,                 // Estrellas del comentario
    #[serde(default)]
    pub like: u32, // Likes del comentario
    #[serde(default)]
    pub reply: Vec<ReplyComment>, // Respuestas al comentario
    #[serde(default)]
    pub users_liked: Vec<String>, // Usuarios que le dieron like
}

/// Constestación de comentarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyComment {
    pub id: String,
    pub author_uid: String, // Usuario que comento
    pub name: String,       // Nombre del usuario
    pub timestamp: String,  // Fecha del comentario
    pub content: String,    // Contenido del comentario
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_img: Option<String>, // Imagen del usuario
    #[serde(default)]
    pub like: u32, // Likes del comentario
    #[serde(default)]
    pub users_liked: Vec<String>, // Usuarios que le dieron like
}

/// Actualización típica de comentario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateComment {
    pub content: String,
    pub stars: f32,
}
