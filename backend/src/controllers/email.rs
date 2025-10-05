use axum::debug_handler;
use tracing::instrument;

use {
    axum::{Json, extract::State, http::StatusCode, response::IntoResponse},
    ecow::EcoString,
    resend_rs::types::CreateEmailBaseOptions,
    std::sync::Arc,
    tracing::{error},
};

use crate::{
    models::{
        email::{CreateEmailResponsePersonalized, EmailIdPersonalized, EmailResend},
        response::ResponseAPI,
    },
    state::AppState,
};

#[debug_handler]
#[instrument(skip(state, payload))]
pub async fn send_contact(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EmailResend>,
) -> impl IntoResponse {
    let html_content: String = format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Correo Electrónico</title>
        <style>
        body {{
            font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            margin: 0;
            padding: 0;
            background-color: #f5f7fa;
    }}
    .email-container {{
        max-width: 600px;
        margin: 0 auto;
        background-color: #ffffff;
        border-radius: 8px;
        overflow: hidden;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
    }}
    .email-header {{
        background: linear-gradient(to right, #f88a7e, #a32e19);
        color: white;
        padding: 30px;
        text-align: center;
    }}
    .email-header h1 {{
        margin: 0;
        font-size: 24px;
    }}
    .email-body {{
        padding: 30px;
    }}

      .email-content {{
        background-color: #f8f9fa;
        padding: 20px;
        border-radius: 6px;
        margin-bottom: 20px;
    }}

      .email-footer {{
        background-color: #f1f3f4;
        padding: 20px;
        text-align: center;
        font-size: 14px;
        color: #6c757d;
    }}

      .sender-info {{
        margin-bottom: 20px;
        padding-bottom: 15px;
        border-bottom: 1px solid #e9ecef;
    }}

      .sender-info p {{
        margin: 5px 0;
    }}

    .message-content {{
        line-height: 1.8;
    }}

      .contact-info {{
        margin-top: 15px;
        font-size: 14px;
    }}

    @media (max-width: 600px) {{
        .email-container {{
          margin: 10px;
    }}

    .email-header,
    .email-body,
    .email-footer {{
        padding: 20px;
    }}
    }}
    </style>
        </head>
        <body>
            <div class="email-container">
                <div class="email-header">
                    <h1>Amanah Academia</h1>
                </div>
                
                <div class="email-body">
                    <div class="sender-info">
                        <p><strong>De:</strong> {} &lt;{}&gt;</p>
                        <p><strong>Asunto:</strong> {}</p>
                    </div>
                    
                    <div class="email-content">
                        <div class="message-content">
                            {}
                        </div>
                    </div>
                </div>
                
                <div class="email-footer">
                    <p>© 2023 Amanah Academia. Todos los derechos reservados.</p>
                    <p class="contact-info">
                        <strong>Contacto:</strong> contact@amanahacademia.com<br>
                        <strong>Teléfono:</strong> +34 123 456 789
                    </p>
                </div>
            </div>
        </body>
        </html>
        "#,
        payload.name,
        payload
            .from
            .as_ref()
            .unwrap_or(&"contact@amanahacademia.com".to_string()),
        payload.subject,
        payload.text
    );

    let email: CreateEmailBaseOptions = CreateEmailBaseOptions::new(
        "contact@amanahacademia.com".to_string(), // from (remitente)
        vec!["contact@amanahacademia.com".to_string()], // to (destinatarios)
        &payload.subject,                         // subject
    )
    .with_html(html_content.as_str());

    match state.resend_client.emails.send(email).await {
        Ok(response) => {
            let response = CreateEmailResponsePersonalized {
                id: EmailIdPersonalized(EcoString::from(response.id.to_string())),
            };
            (
                StatusCode::OK,
                Json(ResponseAPI::<CreateEmailResponsePersonalized>::success(
                    "Email sent successfully".to_string(),
                    response,
                )),
            )
                .into_response()
        }
        Err(e) => {
            error!("[ERROR] Failed to send email: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error("Failed to send email".to_string())),
            )
                .into_response()
        }
    }
}
