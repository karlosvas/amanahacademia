use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub cal_link: String,
    pub name: String,
    pub native_lang: String,
    pub url_image: String,
    pub description: String,
}
