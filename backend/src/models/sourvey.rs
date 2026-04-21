use serde::{Deserialize, Serialize};

/// Nivel de espanol del usuario. En frontend se permite un string libre,
/// por eso en backend se modela como alias para mantener flexibilidad.
pub type SpanishLevel = String;

/// Area de enfoque de la encuesta. Se permite string libre.
pub type FocusArea = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    Text,
    Number,
    Radio,
    Checkbox,
    Textarea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub options: Option<Vec<String>>,
    pub required: bool,
    pub answer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    pub id: String,
    pub title: String,
    pub description: String,
    pub user_email: String,
    pub submitted_at: Option<String>,
    pub questions: Vec<Question>,
}
