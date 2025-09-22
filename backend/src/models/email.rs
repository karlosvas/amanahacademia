use ecow::EcoString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailResend {
    pub from: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    pub subject: String,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateEmailResponsePersonalized {
    pub id: EmailIdPersonalized,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailIdPersonalized(pub EcoString);
