use {
    crate::{
        models::{
            mailchimp::{AddContactResponse, Contact, MembersResponse, UpsertContact},
            response::ResponseAPI,
            state::AppState,
        },
        services::mailchimp::handle_mailchimp_response,
    },
    axum::{
        extract::{Json, State},
        http::StatusCode,
        response::IntoResponse,
    },
    std::sync::Arc,
};

pub async fn add_contact(
    State(state): State<Arc<AppState>>,
    Json(contact): Json<Contact>,
) -> impl IntoResponse {
    let email: String = contact.email_address.trim().to_lowercase();
    let url_mailchimp: String = format!(
        "{}/lists/{}/members/{}",
        state.mailchimp_client.get_base_url(),
        state.mailchimp_client.list_id,
        email
    );
    let body = UpsertContact {
        email_address: &email,
        status_if_new: &contact.status,
        merge_fields: contact.merge_fields.as_ref(),
    };

    match state
        .mailchimp_client
        .client
        .put(&url_mailchimp)
        .basic_auth("", Some(&state.mailchimp_client.api_key))
        .json(&body)
        .send()
        .await
    {
        Ok(response) => match handle_mailchimp_response::<AddContactResponse>(response).await {
            Ok(contact_response) => {
                let message = if contact_response.status == "subscribed"
                    || contact_response.status == "pending"
                {
                    "Contacto añadido correctamente"
                } else {
                    "El contacto ya existía y no está suscrito"
                };
                (
                    StatusCode::OK,
                    Json(ResponseAPI::<AddContactResponse>::success(
                        message.to_string(),
                        contact_response,
                    )),
                )
                    .into_response()
            }
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(format!("Request error: {e}"))),
        )
            .into_response(),
    }
}

pub async fn get_all_contacts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // URL para obtener todos los miembros de la lista
    let url_mailchimp: String = format!(
        "{}/lists/{}/members",
        state.mailchimp_client.get_base_url(),
        state.mailchimp_client.list_id
    );

    match state
        .mailchimp_client
        .client
        .get(&url_mailchimp)
        .basic_auth("", Some(&state.mailchimp_client.api_key))
        .query(&[("count", "1000")]) // Opcional: limitar resultados
        .send()
        .await
    {
        Ok(response) => match handle_mailchimp_response::<MembersResponse>(response).await {
            Ok(members_response) => (
                StatusCode::OK,
                Json(ResponseAPI::<MembersResponse>::success(
                    "Contactos obtenidos correctamente".to_string(),
                    members_response,
                )),
            )
                .into_response(),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(format!("Request error: {e}"))),
        )
            .into_response(),
    }
}

#[cfg(test)]
#[path = "../test/controllers/mailchimp.rs"]
mod extended_tests;
