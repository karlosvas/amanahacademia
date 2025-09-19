use serde::{Deserialize, Serialize};

// Para recibir la petición del cliente
#[derive(Deserialize, Debug)]
pub struct SessionRequest {
    pub token: String, // Solo el token del cliente
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SessionData {
    pub token: String,
    pub local_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub exp: u64,
    pub picture: Option<String>,
    pub email_verified: bool,
    pub provider: Option<String>,
}
