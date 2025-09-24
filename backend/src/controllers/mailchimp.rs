use {
    crate::{
        models::{
            mailchimp::{AddContactResponse, Contact, ErrorMailchimp},
            response::ResponseAPI,
        },
        services::mailchimp::handle_mailchimp_response,
        state::AppState,
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
    // URL para agreagar contactos a nuestra audiencia
    let url_mailchimp: String = format!(
        "{}/lists/{}/members",
        state.mailchimp_client.get_base_url(),
        state.mailchimp_client.list_id
    );

    match state
        .mailchimp_client
        .client
        .post(&url_mailchimp)
        .basic_auth("", Some(&state.mailchimp_client.api_key))
        .json(&contact)
        .send()
        .await
    {
        Ok(response) => match handle_mailchimp_response::<AddContactResponse>(response).await {
            Ok(contact_response) => (
                StatusCode::CREATED,
                Json(ResponseAPI::<AddContactResponse>::success(
                    "Contacto añadido correctamente".to_string(),
                    contact_response,
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

// pub async fn get_lists(
//     State(state): State<Arc<AppState>>,
//     Json(contact): Json<Contact>,
// ) -> impl IntoResponse {
//     let url_mailchimp: String = format!("{}/lists", state.mailchimp_client.get_base_url());

//     let response = state
//         .mailchimp_client
//         .client
//         .get(&url)
//         .basic_auth("", Some(&state.mailchimp_client.api_key))
//         .send()
//         .await?;

//     if response.status().is_success() {
//         let lists: ListsResponse = response.json().await?;
//         Ok(lists)
//     } else {
//         let error_text = response.text().await?;
//         anyhow::bail!("Mailchimp API error: {}", error_text)
//     }
// }
