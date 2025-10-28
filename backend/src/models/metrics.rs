use serde::{Deserialize, Serialize};

// Wrapper para el token de Google Analytics en las extensiones
#[derive(Clone)]
pub struct GAToken(pub String);

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct ServiceAccount {
    pub client_email: String,
    pub private_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct GAResponse {
    #[serde(default)]
    pub rows: Vec<MetricData>,
}

#[derive(Deserialize, Debug)]
pub struct GAErrorResponse {
    pub error: GAError,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct GAError {
    pub code: i32,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Serialize)]
pub struct MetricData {
    #[serde(rename = "dimensionValues")]
    dimension_values: Vec<DimensionValue>,
    #[serde(rename = "metricValues")]
    metric_values: Vec<MetricValue>,
}

#[derive(Deserialize, Serialize)]
struct DimensionValue {
    value: String, // "202410" formato YYYYMM
}

#[derive(Deserialize, Serialize)]
struct MetricValue {
    value: String, // "1234" como string
}

#[derive(Serialize)]
pub struct ClaimsGA {
    pub iss: String,
    pub scope: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
}
