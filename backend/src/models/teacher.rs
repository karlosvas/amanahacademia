use serde::{Deserialize, Serialize};

/// Modelo que representa a un profesor
#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub cal_link: String,
    pub cal_id: String,
    pub name: String,
    pub native_lang: String,
    pub url_image: String,
    pub description: String,
}
