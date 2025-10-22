use serde::{Deserialize, Serialize};

/// Email de resend
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailResend {
    pub to: Vec<String>,
    pub name: String,
    pub subject: String,
    pub text: String,
}
